--
-- Recovery-specific atomic observation
--

local DRAW_ARGUMENT_NOT_REQUESTED = 1
local DRAW_ARGUMENT_OBSERVED = 2
local DRAW_ARGUMENT_UNAVAILABLE = 3

-- Wall-clock milliseconds for measuring the callback's own execution time. Prefers the DLL's
-- monotonic clock; falls back to `os.clock()` (CPU time) when the mission environment still
-- exposes `os`; returns nil when neither is available.
local function clockMs()
  if type(GRPC.monotonicMs) == "function" then
    local ok, value = pcall(GRPC.monotonicMs)
    if ok and type(value) == "number" then
      return value
    end
  end
  if os ~= nil and type(os.clock) == "function" then
    return os.clock() * 1000
  end
  return nil
end

GRPC.methods.getRecoverySnapshot = function(params, meta)
  local startedAt = clockMs()

  if params.carrierName == nil or params.carrierName == "" then
    return GRPC.errorInvalidArgument("carrierName must be provided")
  end
  if params.aircraftName == nil or params.aircraftName == "" then
    return GRPC.errorInvalidArgument("aircraftName must be provided")
  end

  local carrier = Unit.getByName(params.carrierName)
  if carrier == nil then
    return GRPC.errorNotFound("carrier unit does not exist")
  end

  local aircraft = Unit.getByName(params.aircraftName)
  if aircraft == nil then
    return GRPC.errorNotFound("aircraft unit does not exist")
  end

  local observedAt = timer.getTime()
  local drawArgument = {
    status = DRAW_ARGUMENT_NOT_REQUESTED
  }

  if params.aircraftDrawArgument ~= nil then
    local ok, value = pcall(function()
      return aircraft:getDrawArgumentValue(params.aircraftDrawArgument)
    end)
    if ok and type(value) == "number" then
      drawArgument.status = DRAW_ARGUMENT_OBSERVED
      drawArgument.value = value
    else
      drawArgument.status = DRAW_ARGUMENT_UNAVAILABLE
      if ok then
        drawArgument.detail = "getDrawArgumentValue returned " .. type(value)
      else
        drawArgument.detail = tostring(value)
      end
    end
  end

  local carrierRawTransform = GRPC.exporters.rawTransform(carrier)
  local aircraftRawTransform = GRPC.exporters.rawTransform(aircraft)

  -- Diagnostics: queue metadata comes from the DLL (see `handleRequest` in grpc.lua); the
  -- execution time covers everything from callback entry until the observation was collected.
  local queueWaitMs, queueDepth
  if type(meta) == "table" then
    if type(meta.queueWaitMs) == "number" then
      queueWaitMs = meta.queueWaitMs
    end
    if type(meta.queueDepthAtEnqueue) == "number" then
      queueDepth = meta.queueDepthAtEnqueue
    end
  end
  local luaExecMs
  if startedAt ~= nil then
    local finishedAt = clockMs()
    if finishedAt ~= nil then
      luaExecMs = math.max(0, finishedAt - startedAt)
    end
  end

  return GRPC.success({
    time = observedAt,
    carrierRawTransform = carrierRawTransform,
    aircraftRawTransform = aircraftRawTransform,
    aircraftDrawArgument = drawArgument,
    sequence = params.sequence,
    queueWaitMs = queueWaitMs,
    luaExecMs = luaExecMs,
    queueDepth = queueDepth,
    dequeuedModelTime = observedAt,
  })
end
