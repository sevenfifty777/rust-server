local M = dofile("lua/DCS-gRPC/recovery_telemetry.lua")

local function setup(options)
  options = options or {}
  local now = 0
  local units = {
    carrier = { id = 100, name = "carrier" },
    carrier2 = { id = 200, name = "carrier2" },
    aircraft1 = { id = 1, name = "aircraft1" },
    aircraft2 = { id = 2, name = "aircraft2" },
  }
  local reads = { carrier = 0, carrier2 = 0, aircraft1 = 0, aircraft2 = 0 }
  local failReads = {}
  local function raw(unit)
    reads[unit.name] = reads[unit.name] + 1
    if failReads[unit.name] then error("injected read failure") end
    return {
      position = { lat = 1, lon = 2, alt = 3, u = 4, v = 5 },
      positionNorth = { x = 1, y = 0, z = 0 },
      forward = { x = 1, y = 0, z = 0 },
      right = { x = 0, y = 0, z = 1 },
      up = { x = 0, y = 1, z = 0 },
      velocity = { x = 0, y = 0, z = 1 },
    }
  end
  local scheduled = 0
  local state = M.new({
    sourceEpoch = "epoch-test",
    config = {
      periodSeconds = 0.05,
      retentionSeconds = options.retentionSeconds or 30,
      capacity = options.capacity or 600,
      leaseSeconds = options.leaseSeconds or 60,
      maxActiveRecoveries = options.maxActiveRecoveries or 16,
      maxActiveCarriers = options.maxActiveCarriers or 8,
      maxBatchSize = 100,
      readsPerSecond = 100,
    },
    now = function() return now end,
    getUnitByName = function(name) return units[name] end,
    getUnitId = function(unit) return unit.id end,
    exportRawTransform = raw,
    ensureScheduled = function()
      scheduled = scheduled + 1
      if options.scheduleFails then return nil, "injected scheduling failure" end
      return true
    end,
  })
  local context = { state = state, units = units, reads = reads, failReads = failReads }
  function context.scheduled() return scheduled end
  function context.setNow(value) now = value end
  function context.start(handle, aircraft, id, carrier, carrierId)
    return M.start(state, {
      owner = "test-owner", recoveryHandle = handle,
      aircraftName = aircraft, aircraftId = id,
      carrierName = carrier or "carrier", carrierId = carrierId or 100,
    })
  end
  function context.read(handle, after, limit, epoch, owner)
    return M.read(state, {
      owner = owner or "test-owner", recoveryHandle = handle,
      expectedSourceEpoch = epoch or "epoch-test",
      afterSequence = after or 0, limit = limit or 100,
    })
  end
  return context
end

local function testRingRetryAndCarrierSharing()
  local c = setup({ capacity = 3 })
  assert(M.captureTick(c.state, 0) == false, "idle engine must not capture")
  assert(c.reads.carrier == 0 and c.scheduled() == 0)
  assert(c.start("r1", "aircraft1", 1).alreadyActive == false)
  assert(c.scheduled() == 1)
  assert(c.start("r1", "aircraft1", 1).alreadyActive == true)
  local _, conflict = c.start("r1", "aircraft2", 2)
  assert(conflict == "ALREADY_EXISTS")
  assert(c.start("r2", "aircraft2", 2))
  for i = 1, 5 do
    c.setNow(i * 0.05)
    assert(M.captureTick(c.state, i * 0.05))
  end
  assert(c.reads.carrier == 5, "carrier must be read once per callback")
  assert(c.reads.aircraft1 == 5 and c.reads.aircraft2 == 5)
  local batch = assert(c.read("r1", 0, 2))
  assert(batch.oldestAvailableSequence == 3 and batch.newestAvailableSequence == 5)
  assert(batch.lossReason == 3 and batch.overflowCount == 2)
  assert(#batch.snapshots == 2 and batch.snapshots[1].sequence == 3)
  assert(batch.nextAfterSequence == 4)
  local retry = assert(c.read("r1", 0, 2))
  assert(retry.snapshots[1].sequence == 3, "read must be non-destructive")
end

local function testLimitsRetentionAndIndependentFailures()
  local limited = setup({ maxActiveRecoveries = 2, maxActiveCarriers = 1 })
  assert(limited.start("r1", "aircraft1", 1))
  assert(limited.start("r2", "aircraft2", 2))
  local _, recoveryLimit = limited.start("r3", "aircraft2", 2)
  assert(recoveryLimit == "RESOURCE_EXHAUSTED")

  local carriers = setup({ maxActiveCarriers = 1 })
  assert(carriers.start("r1", "aircraft1", 1))
  local _, carrierLimit = carriers.start("r2", "aircraft2", 2, "carrier2", 200)
  assert(carrierLimit == "RESOURCE_EXHAUSTED")

  local failures = setup()
  assert(failures.start("r1", "aircraft1", 1))
  assert(failures.start("r2", "aircraft2", 2))
  failures.failReads.aircraft1 = true
  failures.setNow(0.05)
  assert(M.captureTick(failures.state, 0.05))
  assert(failures.read("r1", 0).snapshots[1].aircraft.status == 4)
  assert(failures.read("r2", 0).snapshots[1].aircraft.status == 1,
    "one aircraft failure must not block another recovery")

  local retention = setup({ retentionSeconds = 1 })
  assert(retention.start("r1", "aircraft1", 1))
  retention.setNow(0.05)
  assert(M.captureTick(retention.state, 0.05))
  retention.setNow(1.1)
  assert(M.captureTick(retention.state, 1.1))
  local aged = retention.read("r1", 0)
  assert(aged.oldestAvailableSequence == 2 and aged.lossReason == 2)
  assert(aged.diagnostics.retentionExpirationCount == 1)

  local paged = setup()
  assert(paged.start("r1", "aircraft1", 1))
  for i = 1, 101 do
    local captureTime = i * 0.01
    paged.setNow(captureTime)
    assert(M.captureTick(paged.state, captureTime))
  end
  assert(#paged.read("r1", 0, 1000).snapshots == 100,
    "server batch limit must clamp oversized requests")
end

local function testDeliveryDelayAndProducerFreeze()
  local c = setup()
  assert(c.start("r1", "aircraft1", 1))
  for i = 1, 3 do
    c.setNow(i * 0.05)
    assert(M.captureTick(c.state, i * 0.05))
  end
  assert(#assert(c.read("r1", 0)).snapshots == 3)
  c.setNow(1.15)
  assert(M.captureTick(c.state, 1.15))
  local resumed = assert(c.read("r1", 3))
  assert(#resumed.snapshots == 1 and resumed.snapshots[1].sequence == 4)
  assert(math.abs(resumed.diagnostics.observedGap - 1.0) < 0.000001)
  assert(resumed.diagnostics.missedCaptureIntervals == 19)
end

local function testEmptyRangeIdleResetAndSchedulingFailure()
  local c = setup()
  assert(c.start("r1", "aircraft1", 1))
  local empty = assert(c.read("r1", 0))
  assert(empty.oldestAvailableSequence == 0 and empty.newestAvailableSequence == 0,
    "an empty ring must expose an explicit 0/0 range")
  c.setNow(0.05)
  assert(M.captureTick(c.state, 0.05))
  assert(M.stop(c.state, {
    owner = "test-owner", recoveryHandle = "r1", expectedSourceEpoch = "epoch-test",
  }))
  c.setNow(20)
  local _, tombstoneConflict = c.start("r1", "aircraft1", 1)
  assert(tombstoneConflict == "ALREADY_EXISTS", "live tombstones must prevent handle reuse")
  assert(c.start("r2", "aircraft1", 1))
  c.setNow(20.05)
  assert(M.captureTick(c.state, 20.05))
  local resumed = assert(c.read("r2", 0))
  assert(resumed.diagnostics.observedGap == nil)
  assert(resumed.diagnostics.missedCaptureIntervals == 0,
    "idle time must not be reported as a producer freeze")

  local failed = setup({ scheduleFails = true })
  local result, kind = failed.start("r1", "aircraft1", 1)
  assert(result == nil and kind == "INTERNAL")
  assert(next(failed.state.recoveries) == nil, "failed scheduling must roll back registration")
end

local function testMixedLossProvenance()
  local c = setup({ capacity = 3, retentionSeconds = 30 })
  assert(c.start("r1", "aircraft1", 1))
  for i = 1, 4 do
    c.setNow(i * 0.05)
    assert(M.captureTick(c.state, i * 0.05))
  end
  c.setNow(31)
  assert(M.captureTick(c.state, 31))
  local mixed = assert(c.read("r1", 0))
  assert(mixed.oldestAvailableSequence == 5 and mixed.lossReason == 4,
    "a cursor spanning both eviction causes must report mixed loss")
  local retentionOnly = assert(c.read("r1", 1))
  assert(retentionOnly.lossReason == 2,
    "the reported cause must match the missing sequences after the cursor")
end

local function testInvalidLifecycleRetentionAndIsolation()
  local c = setup({ retentionSeconds = 1, leaseSeconds = 15, maxActiveRecoveries = 2 })
  assert(c.start("r1", "aircraft1", 1))
  c.units.aircraft1.id = 99
  c.setNow(0.05)
  assert(M.captureTick(c.state, 0.05))
  assert(c.read("r1", 0).snapshots[1].aircraft.status == 3)
  local _, kind = c.read("r1", 0, 100, "epoch-test", "other-owner")
  assert(kind == "PERMISSION_DENIED")
  assert(M.stop(c.state, {
    owner = "test-owner", recoveryHandle = "r1", expectedSourceEpoch = "epoch-test",
  }).lifecycleStatus == 4)
  assert(M.stop(c.state, {
    owner = "test-owner", recoveryHandle = "r1", expectedSourceEpoch = "epoch-test",
  }).lifecycleStatus == 4)
  assert(c.start("r2", "aircraft2", 2))
  c.setNow(16)
  assert(c.read("r2", 0).lifecycleStatus == 3)
  assert(c.read("unknown", 0, 100, "old-epoch").lifecycleStatus == 5)
end

testRingRetryAndCarrierSharing()
testDeliveryDelayAndProducerFreeze()
testEmptyRangeIdleResetAndSchedulingFailure()
testMixedLossProvenance()
testInvalidLifecycleRetentionAndIsolation()
testLimitsRetentionAndIndependentFailures()
print("recovery telemetry Lua tests passed")
