std = "lua51"
globals = {
  "GRPC",
  "grpc",
}
read_globals = {
  "AI",
  "Airbase",
  "atmosphere",
  "coalition",
  "net",
  "coord",
  "DCS",
  "env",
  "Group",
  "land",
  "lfs",
  "log",
  "Object",
  "StaticObject",
  "Spot",
  "timer",
  "trigger",
  "Unit",
  "world",
  "missionCommands",
  "Export"
}

-- Avoid rewriting this legacy mixed-line-ending file for two whitespace-only lines.
files["lua/DCS-gRPC/methods/unit.lua"] = {
  ignore = {
    "611",
  },
}
