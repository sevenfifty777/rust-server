use mlua::Lua;

fn run_harness(body: &str) {
    let lua = Lua::new();
    let engine = include_str!("../lua/DCS-gRPC/recovery_telemetry.lua");
    let script = format!("local M = (function()\n{engine}\nend)()\n{body}");
    lua.load(&script).exec().unwrap();
}

const SETUP: &str = r#"
local now = 0
local units = {
  carrier = { id = 100 },
  aircraft1 = { id = 1 },
  aircraft2 = { id = 2 },
}
local reads = { carrier = 0, aircraft1 = 0, aircraft2 = 0 }
local function raw(unit)
  reads[unit.name] = reads[unit.name] + 1
  return {
    position = { lat = 1, lon = 2, alt = 3, u = 4, v = 5 },
    positionNorth = { x = 1, y = 0, z = 0 },
    forward = { x = 1, y = 0, z = 0 },
    right = { x = 0, y = 0, z = 1 },
    up = { x = 0, y = 1, z = 0 },
    velocity = { x = 0, y = 0, z = 1 },
  }
end
for name, unit in pairs(units) do unit.name = name end
local state = M.new({
  sourceEpoch = "epoch-test",
  config = {
    periodSeconds = 0.05,
    retentionSeconds = 30,
    capacity = 3,
    leaseSeconds = 60,
    maxActiveRecoveries = 16,
    maxActiveCarriers = 8,
    maxBatchSize = 100,
    readsPerSecond = 100,
  },
  now = function() return now end,
  getUnitByName = function(name) return units[name] end,
  getUnitId = function(unit) return unit.id end,
  exportRawTransform = raw,
})
local function start(handle, aircraft, id)
  return M.start(state, {
    owner = "test-owner", recoveryHandle = handle,
    aircraftName = aircraft, aircraftId = id,
    carrierName = "carrier", carrierId = 100,
  })
end
"#;

#[test]
fn ring_pagination_overflow_and_shared_carrier_are_deterministic() {
    run_harness(
        &[
            SETUP,
            r#"
local first = assert(start("r1", "aircraft1", 1))
assert(first.alreadyActive == false and first.sourceEpoch == "epoch-test")
local duplicate = assert(start("r1", "aircraft1", 1))
assert(duplicate.alreadyActive == true)
assert(start("r2", "aircraft2", 2))
for i = 1, 5 do now = i * 0.05; assert(M.captureTick(state, now)) end
assert(reads.carrier == 5, "carrier must be read once per callback")
assert(reads.aircraft1 == 5 and reads.aircraft2 == 5)
local batch = assert(M.read(state, {
  owner = "test-owner", recoveryHandle = "r1", expectedSourceEpoch = "epoch-test",
  afterSequence = 0, limit = 2,
}))
assert(batch.oldestAvailableSequence == 3 and batch.newestAvailableSequence == 5)
assert(batch.lossReason == 3 and batch.overflowCount == 2)
assert(#batch.snapshots == 2 and batch.snapshots[1].sequence == 3)
assert(batch.nextAfterSequence == 4)
local retry = assert(M.read(state, {
  owner = "test-owner", recoveryHandle = "r1", expectedSourceEpoch = "epoch-test",
  afterSequence = 0, limit = 2,
}))
assert(retry.snapshots[1].sequence == 3, "read must be non-destructive and idempotent")
"#,
        ]
        .concat(),
    );
}

#[test]
fn delivery_delay_and_producer_freeze_have_distinct_results() {
    run_harness(
        &[
            SETUP,
            r#"
assert(start("r1", "aircraft1", 1))
for i = 1, 3 do now = i * 0.05; assert(M.captureTick(state, now)) end
local delayed = assert(M.read(state, {
  owner = "test-owner", recoveryHandle = "r1", expectedSourceEpoch = "epoch-test",
  afterSequence = 0, limit = 100,
}))
assert(#delayed.snapshots == 3, "late delivery must not erase captured samples")
now = 1.15
assert(M.captureTick(state, now))
local resumed = assert(M.read(state, {
  owner = "test-owner", recoveryHandle = "r1", expectedSourceEpoch = "epoch-test",
  afterSequence = 3, limit = 100,
}))
assert(#resumed.snapshots == 1 and resumed.snapshots[1].sequence == 4)
assert(resumed.diagnostics.observedGap == 1.0)
assert(resumed.diagnostics.missedCaptureIntervals == 19)
"#,
        ]
        .concat(),
    );
}

#[test]
fn empty_range_idle_restart_and_mixed_loss_are_explicit() {
    run_harness(
        &[
            SETUP,
            r#"
assert(start("r1", "aircraft1", 1))
local empty = assert(M.read(state, {
  owner = "test-owner", recoveryHandle = "r1", expectedSourceEpoch = "epoch-test",
  afterSequence = 0, limit = 100,
}))
assert(empty.oldestAvailableSequence == 0 and empty.newestAvailableSequence == 0)
now = 0.05
assert(M.captureTick(state, now))
assert(M.stop(state, {
  owner = "test-owner", recoveryHandle = "r1", expectedSourceEpoch = "epoch-test",
}))
now = 20
local reused, reuseKind = start("r1", "aircraft1", 1)
assert(reused == nil and reuseKind == "ALREADY_EXISTS")
assert(start("r2", "aircraft1", 1))
now = 20.05
assert(M.captureTick(state, now))
local resumed = assert(M.read(state, {
  owner = "test-owner", recoveryHandle = "r2", expectedSourceEpoch = "epoch-test",
  afterSequence = 0, limit = 100,
}))
assert(resumed.diagnostics.observedGap == nil)
assert(resumed.diagnostics.missedCaptureIntervals == 0)

-- Capacity first evicts sequence 1, then retention evicts sequences 2..4.
local mixedState = M.new({
  sourceEpoch = "epoch-mixed",
  config = state.config,
  now = function() return now end,
  getUnitByName = state.getUnitByName,
  getUnitId = state.getUnitId,
  exportRawTransform = state.exportRawTransform,
})
local function startMixed()
  return M.start(mixedState, {
    owner = "test-owner", recoveryHandle = "mixed", aircraftName = "aircraft1",
    aircraftId = 1, carrierName = "carrier", carrierId = 100,
  })
end
now = 0
assert(startMixed())
for i = 1, 4 do now = i * 0.05; assert(M.captureTick(mixedState, now)) end
now = 31
assert(M.captureTick(mixedState, now))
local mixed = assert(M.read(mixedState, {
  owner = "test-owner", recoveryHandle = "mixed", expectedSourceEpoch = "epoch-mixed",
  afterSequence = 0, limit = 100,
}))
assert(mixed.oldestAvailableSequence == 5 and mixed.lossReason == 4)
local retentionOnly = assert(M.read(mixedState, {
  owner = "test-owner", recoveryHandle = "mixed", expectedSourceEpoch = "epoch-mixed",
  afterSequence = 1, limit = 100,
}))
assert(retentionOnly.lossReason == 2)
"#,
        ]
        .concat(),
    );
}

#[test]
fn invalid_incarnation_stop_and_ttl_are_explicit() {
    run_harness(
        &[
            SETUP,
            r#"
assert(start("r1", "aircraft1", 1))
units.aircraft1.id = 99
now = 0.05
assert(M.captureTick(state, now))
local invalid = assert(M.read(state, {
  owner = "test-owner", recoveryHandle = "r1", expectedSourceEpoch = "epoch-test",
  afterSequence = 0, limit = 100,
}))
assert(invalid.snapshots[1].aircraft.status == 3)
local stopped = assert(M.stop(state, {
  owner = "test-owner", recoveryHandle = "r1", expectedSourceEpoch = "epoch-test",
}))
assert(stopped.lifecycleStatus == 4)
local stoppedRetry = assert(M.stop(state, {
  owner = "test-owner", recoveryHandle = "r1", expectedSourceEpoch = "epoch-test",
}))
assert(stoppedRetry.lifecycleStatus == 4)
assert(start("r2", "aircraft2", 2))
now = 61
local expired = assert(M.read(state, {
  owner = "test-owner", recoveryHandle = "r2", expectedSourceEpoch = "epoch-test",
  afterSequence = 0, limit = 100,
}))
assert(expired.lifecycleStatus == 3)
local mismatch = assert(M.read(state, {
  owner = "test-owner", recoveryHandle = "unknown", expectedSourceEpoch = "old-epoch",
  afterSequence = 0, limit = 100,
}))
assert(mismatch.lifecycleStatus == 5)
"#,
        ]
        .concat(),
    );
}
