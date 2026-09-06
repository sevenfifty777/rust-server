local now = 0
local failClock = false
local failSchedule = false
local scheduledCallback = nil
local errors = {}

timer = {
  getTime = function()
    if failClock then error("injected clock failure") end
    return now
  end,
  scheduleFunction = function(callback)
    if failSchedule then error("injected scheduling failure") end
    scheduledCallback = callback
    return 42
  end,
}

local units = {
  aircraft = { id = 1 },
  carrier = { id = 100 },
}
Unit = {
  getByName = function(name) return units[name] end,
}

local function rawTransform()
  return {
    position = { lat = 1, lon = 2, alt = 3, u = 4, v = 5 },
    positionNorth = { x = 1, y = 0, z = 0 },
    forward = { x = 1, y = 0, z = 0 },
    right = { x = 0, y = 0, z = 1 },
    up = { x = 0, y = 1, z = 0 },
    velocity = { x = 0, y = 0, z = 1 },
  }
end

GRPC = {
  recoveryTelemetry = { enabled = true },
  isMissionEnv = true,
  luaPath = "lua/DCS-gRPC/",
  methods = {},
  exporters = { rawTransform = rawTransform },
  success = function(value) return value end,
  error = function(message) return { errorKind = "INTERNAL", message = message } end,
  errorInvalidArgument = function(message) return { errorKind = "INVALID_ARGUMENT", message = message } end,
  errorAlreadyExists = function(message) return { errorKind = "ALREADY_EXISTS", message = message } end,
  errorPermissionDenied = function(message) return { errorKind = "PERMISSION_DENIED", message = message } end,
  errorResourceExhausted = function(message) return { errorKind = "RESOURCE_EXHAUSTED", message = message } end,
  errorUnimplemented = function(message) return { errorKind = "UNIMPLEMENTED", message = message } end,
  logInfo = function() end,
  logError = function(message) table.insert(errors, message) end,
  -- Native helpers are exposed on GRPC by grpc.lua (not as a global `grpc`).
  newSessionId = function() return "epoch-scheduler-test" end,
  monotonicTimeNs = function() return 1000000 end,
}

local function start()
  return GRPC.methods.startRecoveryTelemetry({
    owner = "test-owner",
    recoveryHandle = "r1",
    aircraftName = "aircraft",
    aircraftId = 1,
    carrierName = "carrier",
    carrierId = 100,
  })
end

dofile("lua/DCS-gRPC/methods/recovery.lua")
assert(start().alreadyActive == false)
assert(type(scheduledCallback) == "function")

failClock = true
local retryAt = scheduledCallback(nil, 0.05)
assert(retryAt == 0.1, "an unexpected callback exception must reschedule capture")
assert(#errors == 1)

failClock = false
now = 0.1
assert(math.abs(scheduledCallback(nil, 0.1) - 0.15) < 0.000001)
local batch = GRPC.methods.readRecoveryTelemetry({
  owner = "test-owner",
  recoveryHandle = "r1",
  expectedSourceEpoch = "epoch-scheduler-test",
  afterSequence = 0,
  limit = 100,
})
assert(#batch.snapshots == 1 and batch.snapshots[1].sequence == 1,
  "capture must recover after a transient callback exception")

-- Reload the method state to exercise initial scheduling failure and rollback.
failSchedule = true
scheduledCallback = nil
dofile("lua/DCS-gRPC/methods/recovery.lua")
local failed = start()
assert(failed.errorKind == "INTERNAL")
failSchedule = false
local retried = start()
assert(retried.alreadyActive == false and type(scheduledCallback) == "function",
  "a scheduling failure must not leave a ghost registration")

print("recovery telemetry scheduler tests passed")
