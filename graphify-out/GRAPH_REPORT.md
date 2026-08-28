# Graph Report - rust-server  (2026-08-28)

## Corpus Check
- Corpus is ~44,827 words - fits in a single context window. You may not need a graph.

## Summary
- 1451 nodes · 2781 edges · 126 communities (90 shown, 36 thin omitted)
- Extraction: 100% EXTRACTED · 0% INFERRED · 0% AMBIGUOUS · INFERRED: 8 edges (avg confidence: 0.84)
- Token cost: 18,533 input · 2,033 output

## Community Hubs (Navigation)
- Server Admin & Coalition API
- Core Event & Group Types
- Unit Ammo & Description API
- Generic Type Conversions
- SRS Client Serialization
- Coalition & Unit Query API
- Command Menu System
- Controller Service
- Airbase Service
- Warehouse & Inventory API
- Lua Bridge & REPL
- Unit Activation & Emission
- Spot & Laser Service
- Land & Net Service
- Static Object Spawning
- TTS Error Handling
- Async Shutdown & Streams
- Custom & Mission Service
- SRS Message Codec
- Hot Reload System
- SRS Voice Codec
- Lua Table & Library Core
- Configuration System
- Player Management API
- Stats & Metrics
- RPC Hook System
- gRPC Server Runtime
- Atmosphere Service
- Timer Service
- SRS Client Networking
- Google Cloud TTS
- REPL Client Tool
- SRS Client State
- Server Error Handling
- Lua gRPC Event Dispatch
- SRS Radio Stream
- Metadata & Health Service
- Group Movement Commands
- Windows TTS Engine
- Auth Interceptor
- Integrity Check
- Group Activation & Cargo
- Arrow & Markup Drawing
- Circle & Text Drawing
- Explosion & Markup Drawing
- Azure TTS Engine
- RPC Request Dispatch
- Config Display Formatting
- Input Position Handling
- Build Script
- Mission Lua Methods
- Stubs Build
- CI & Contributing Docs
- Smoke Big Effect
- Smoke Stop Effect
- User Flag Get
- Zone Query
- Line Drawing
- Mark To All
- Mark To Coalition
- Mark To Group
- Markup To Coalition
- Text For Coalition
- Text For Group
- Text For Unit
- Push AI Task
- Rect Drawing
- Remove Mark
- Set AI Task
- Group AI Off
- Group AI On
- Markup Color
- Markup Font Size
- Markup Position Start
- Markup Radius
- Markup Text
- User Flag Set
- Signal Flare
- Smoke Trigger
- Text To All
- Minor Component 88
- Minor Component 103
- Minor Component 104
- Minor Component 105
- Minor Component 106
- Minor Component 107
- Minor Component 108

## God Nodes (most connected - your core abstractions)
1. `MissionRpc` - 43 edges
2. `HookRpc` - 27 edges
3. `MissionRpc` - 24 edges
4. `ShutdownHandle` - 22 edges
5. `ValueVisitor` - 22 edges
6. `Client` - 18 edges
7. `Config` - 17 edges
8. `Stats` - 17 edges
9. `MissionRpc` - 15 edges
10. `MissionRpc` - 15 edges

## Surprising Connections (you probably didn't know these)
- `TtsOptions` --references--> `Coalition`  [EXTRACTED]
  src/server.rs → srs/src/message.rs
- `run()` --references--> `StreamError`  [EXTRACTED]
  src/srs.rs → srs/src/stream.rs
- `position_equalish()` --references--> `Position`  [EXTRACTED]
  src/stream.rs → srs/src/message.rs
- `RawTransform` --references--> `Position`  [EXTRACTED]
  stubs/src/common.rs → srs/src/message.rs
- `Transform` --references--> `Position`  [EXTRACTED]
  stubs/src/common.rs → srs/src/message.rs

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **gRPC Service Layer** — weaponservice, triggerservice, landservice, unitservice, srsservice, metadataservice, worldservice [EXTRACTED 1.00]
- **Development & Quality Standards** — contributing, protolint_yaml, github_workflows_ci_workflow [INFERRED 0.90]

## Communities (126 total, 36 thin omitted)

### Community 0 - "Server Admin & Coalition API"
Cohesion: 0.05
Nodes (58): BanPlayerRequest, BanPlayerResponse, ExitProcessRequest, ExitProcessResponse, GetAvailableCoalitionsRequest, GetAvailableCoalitionsResponse, GetAvailableSlotsRequest, GetAvailableSlotsResponse (+50 more)

### Community 1 - "Core Event & Group Types"
Cohesion: 0.05
Nodes (72): Duration, Event, Group, GroupCategory, HashSet, SendError, event_time(), Future (+64 more)

### Community 2 - "Unit Ammo & Description API"
Cohesion: 0.05
Nodes (56): GetAmmoRequest, GetAmmoResponse, GetDescByNameRequest, GetDescByNameResponse, GetDescentCapacityRequest, GetDescentCapacityResponse, GetDescriptorRequest, GetDescriptorResponse (+48 more)

### Community 3 - "Generic Type Conversions"
Cohesion: 0.10
Nodes (36): A, E, Into, Kind, M, Struct, create_message(), deserialize() (+28 more)

### Community 4 - "SRS Client Serialization"
Cohesion: 0.08
Nodes (39): Default, Deserialize, Modulation, Serialize, Self, Client, ClientDisconnectMessage, Coalition (+31 more)

### Community 5 - "Coalition & Unit Query API"
Cohesion: 0.08
Nodes (35): GetCoalitionRequest, GetCoalitionResponse, GetDescRequest, GetDescResponse, GetLauncherRequest, GetLauncherResponse, GetNameRequest, GetNameResponse (+27 more)

### Community 6 - "Command Menu System"
Cohesion: 0.09
Nodes (35): AddCoalitionCommandRequest, AddCoalitionCommandResponse, AddCoalitionCommandSubMenuRequest, AddCoalitionCommandSubMenuResponse, AddGroupCommandRequest, AddGroupCommandResponse, AddGroupCommandSubMenuRequest, AddGroupCommandSubMenuResponse (+27 more)

### Community 7 - "Controller Service"
Cohesion: 0.10
Nodes (30): ControllerService, GetDetectedTargetsRequest, GetDetectedTargetsResponse, HasTaskRequest, HasTaskResponse, IsTargetDetectedRequest, IsTargetDetectedResponse, KnowTargetRequest (+22 more)

### Community 8 - "Airbase Service"
Cohesion: 0.09
Nodes (26): GetAirbaseIdRequest, GetAirbaseIdResponse, GetAirbaseParkingRequest, GetAirbaseParkingResponse, GetAirbaseRadioSilentModeRequest, GetAirbaseRadioSilentModeResponse, GetAirbaseRunwaysRequest, GetAirbaseRunwaysResponse (+18 more)

### Community 9 - "Warehouse & Inventory API"
Cohesion: 0.12
Nodes (24): AddItemRequest, AddItemResponse, AddLiquidRequest, AddLiquidResponse, GetInventoryRequest, GetInventoryResponse, GetItemCountRequest, GetItemCountResponse (+16 more)

### Community 10 - "Lua Bridge & REPL"
Cohesion: 0.10
Nodes (28): Release Process, GetClientsRequest, GetClientsResponse, Lua Bridge, DCS-gRPC REPL, Rust gRPC Server, Box, Error (+20 more)

### Community 11 - "Unit Activation & Emission"
Cohesion: 0.12
Nodes (22): ActivateRequest, ActivateResponse, EnableEmissionRequest, EnableEmissionResponse, ExistsRequest, ExistsResponse, GetSizeRequest, GetSizeResponse (+14 more)

### Community 12 - "Spot & Laser Service"
Cohesion: 0.12
Nodes (22): CreateInfraRedRequest, CreateInfraRedResponse, CreateLaserRequest, CreateLaserResponse, GetCodeRequest, GetCodeResponse, SetCodeRequest, SetCodeResponse (+14 more)

### Community 13 - "Land & Net Service"
Cohesion: 0.12
Nodes (22): FindPathOnRoadsRequest, FindPathOnRoadsResponse, GetClosestPointOnRoadsRequest, GetClosestPointOnRoadsResponse, GetIpRequest, GetIpResponse, GetSurfaceHeightWithSeabedRequest, GetSurfaceHeightWithSeabedResponse (+14 more)

### Community 14 - "Static Object Spawning"
Cohesion: 0.13
Nodes (20): AddGroupRequest, AddGroupResponse, AddLinkedStaticRequest, AddLinkedStaticResponse, AddStaticObjectRequest, AddStaticObjectResponse, CoalitionService, GetBullseyeRequest (+12 more)

### Community 15 - "TTS Error Handling"
Cohesion: 0.10
Nodes (26): Bytes, Region, RusotoError, SynthesizeSpeechError, TlsError, AwsConfig, AwsError, Error (+18 more)

### Community 16 - "Async Shutdown & Streams"
Cohesion: 0.11
Nodes (19): Pin, Poll, Shared, AbortableStream, AbortableStream<F, S>, F, Future, Item (+11 more)

### Community 17 - "Custom & Mission Service"
Cohesion: 0.15
Nodes (18): AbortMissionRequest, AbortMissionResponse, CustomService, GetMagneticDeclinationRequest, GetMagneticDeclinationResponse, GetMissionStatusRequest, GetMissionStatusResponse, JoinMissionRequest (+10 more)

### Community 18 - "SRS Message Codec"
Cohesion: 0.14
Nodes (16): LinesCodec, LinesCodecError, MessagesCodec, MessagesCodecError, BytesMut, Decoder, Display, Encoder (+8 more)

### Community 19 - "Hot Reload System"
Cohesion: 0.18
Nodes (22): event(), log_debug(), log_error(), log_info(), log_warning(), next(), Display, Error (+14 more)

### Community 20 - "SRS Voice Codec"
Cohesion: 0.16
Nodes (17): LengthDelimitedCodec, Encryption, Frequency, Modulation, Packet, BytesMut, Decoder, Encoder (+9 more)

### Community 21 - "Lua Table & Library Core"
Cohesion: 0.23
Nodes (22): LuaTable, dcs_grpc(), dcs_grpc_hot_reload(), Error, event(), init(), log_debug(), log_error() (+14 more)

### Community 22 - "Configuration System"
Cohesion: 0.17
Nodes (20): Debug, AwsConfig, AzureConfig, Config, default_host(), GCloudConfig, AwsConfig, AzureConfig (+12 more)

### Community 23 - "Player Management API"
Cohesion: 0.16
Nodes (16): ForcePlayerSlotRequest, ForcePlayerSlotResponse, GetPlayersRequest, GetPlayersResponse, KickPlayerRequest, KickPlayerResponse, NetService, SendChatRequest (+8 more)

### Community 24 - "Stats & Metrics"
Cohesion: 0.16
Nodes (12): AtomicU32, AtomicUsize, Drop, Inner, IntervalStats, Arc, Instant, Mutex (+4 more)

### Community 25 - "RPC Hook System"
Cohesion: 0.18
Nodes (14): Cache, HookRpc, MissionRpc, Arc, IPC, Item, OffsetDateTime, Option (+6 more)

### Community 26 - "gRPC Server Runtime"
Cohesion: 0.18
Nodes (13): Runtime, Arc, F, IPC, Mutex, Output, Sender, SocketAddr (+5 more)

### Community 27 - "Atmosphere Service"
Cohesion: 0.18
Nodes (14): AtmosphereService, GetTemperatureAndPressureRequest, GetTemperatureAndPressureResponse, GetWindRequest, GetWindResponse, GetWindWithTurbulenceRequest, GetWindWithTurbulenceResponse, get_wind_heading_and_strength() (+6 more)

### Community 28 - "Timer Service"
Cohesion: 0.16
Nodes (13): GetAbsoluteTimeRequest, GetAbsoluteTimeResponse, GetTimeRequest, GetTimeResponse, GetTimeZeroRequest, GetTimeZeroResponse, MissionService, MissionRpc (+5 more)

### Community 29 - "SRS Client Networking"
Cohesion: 0.14
Nodes (14): Message, Future, Output, Receiver, Result, Send, Sender, SocketAddr (+6 more)

### Community 30 - "Google Cloud TTS"
Cohesion: 0.18
Nodes (15): DecodeError, AudioConfig, GCloudConfig, GcloudError, Input, Error, OggReadError, Option (+7 more)

### Community 31 - "REPL Client Tool"
Cohesion: 0.21
Nodes (14): CustomServiceClient, HookServiceClient, Client, Error, handle_respone(), main(), Opts, Box (+6 more)

### Community 32 - "SRS Client State"
Cohesion: 0.21
Nodes (7): Client, Arc, Option, RwLock, String, UnitInfo, Position

### Community 33 - "Server Error Handling"
Cohesion: 0.21
Nodes (11): AddrParseError, Handle, Error, Lua, Receiver, Result, Self, Value (+3 more)

### Community 35 - "SRS Radio Stream"
Cohesion: 0.23
Nodes (13): create_radio_update_message(), create_sync_message(), create_update_message(), Client, Future, Output, Receiver, Result (+5 more)

### Community 36 - "Metadata & Health Service"
Cohesion: 0.21
Nodes (10): GetHealthRequest, GetHealthResponse, GetVersionRequest, GetVersionResponse, MetadataService, MissionRpc, Request, Response (+2 more)

### Community 37 - "Group Movement Commands"
Cohesion: 0.17
Nodes (8): DeactivateGroupRequest, DeactivateGroupResponse, GroupStopMovingRequest, GroupStopMovingResponse, IlluminationBombRequest, IlluminationBombResponse, MissionRpc, TriggerService

### Community 38 - "Windows TTS Engine"
Cohesion: 0.24
Nodes (10): Error, From, Option, Result, Self, String, Vec, synthesize() (+2 more)

### Community 39 - "Auth Interceptor"
Cohesion: 0.25
Nodes (9): Body, RequestInterceptor, AuthInterceptor, Request, Result, Status, ApiKey, AuthConfig (+1 more)

### Community 40 - "Integrity Check"
Cohesion: 0.27
Nodes (9): File, PathBuf, check(), file_hash(), IntegrityError, Display, Error, Formatter (+1 more)

### Community 41 - "Group Activation & Cargo"
Cohesion: 0.20
Nodes (7): ActivateGroupRequest, ActivateGroupResponse, GroupContinueMovingRequest, GroupContinueMovingResponse, SetUnitInternalCargoRequest, SetUnitInternalCargoResponse, Response

### Community 42 - "Arrow & Markup Drawing"
Cohesion: 0.20
Nodes (7): ArrowToAllRequest, ArrowToAllResponse, SetMarkupColorFillRequest, SetMarkupColorFillResponse, SetMarkupTypeLineRequest, SetMarkupTypeLineResponse, Result

### Community 43 - "Circle & Text Drawing"
Cohesion: 0.20
Nodes (7): CircleToAllRequest, CircleToAllResponse, OutTextRequest, OutTextResponse, QuadToAllRequest, QuadToAllResponse, Status

### Community 44 - "Explosion & Markup Drawing"
Cohesion: 0.20
Nodes (7): ExplosionRequest, ExplosionResponse, MarkupToAllRequest, MarkupToAllResponse, SetMarkupPositionEndRequest, SetMarkupPositionEndResponse, Request

### Community 45 - "Azure TTS Engine"
Cohesion: 0.27
Nodes (9): AzureConfig, AzureError, Error, OggReadError, Option, Result, String, Vec (+1 more)

### Community 46 - "RPC Request Dispatch"
Cohesion: 0.36
Nodes (7): I, O, Error, Request, Result, Status, to_status()

### Community 47 - "Config Display Formatting"
Cohesion: 0.28
Nodes (5): Formatter, Lua, Result, Self, Value

### Community 48 - "Input Position Handling"
Cohesion: 0.38
Nodes (6): InputPosition, Provider, FromLua, Option, String, TtsOptions

### Community 49 - "Build Script"
Cohesion: 0.60
Nodes (5): embed_lua_file_hashes(), file_hash(), main(), write_version_to_lua(), Path

### Community 52 - "Stubs Build"
Cohesion: 0.40
Nodes (4): main(), Box, Error, Result

### Community 54 - "CI & Contributing Docs"
Cohesion: 0.67
Nodes (3): Contributing Guidelines, CI Workflow, Protolint Configuration

## Knowledge Gaps
- **13 isolated node(s):** `dcs-grpc`, `dcs-grpc-repl`, `dcs-grpc-srs`, `Modulation`, `RadioSwitchControls` (+8 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **36 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `MissionRpc` connect `Group Movement Commands` to `Group Activation & Cargo`, `Arrow & Markup Drawing`, `Circle & Text Drawing`, `Explosion & Markup Drawing`, `Smoke Big Effect`, `Smoke Stop Effect`, `User Flag Get`, `Zone Query`, `Line Drawing`, `Mark To All`, `Mark To Coalition`, `Mark To Group`, `Markup To Coalition`, `Text For Coalition`, `Text For Group`, `Text For Unit`, `Push AI Task`, `Rect Drawing`, `Remove Mark`, `Set AI Task`, `Group AI Off`, `Group AI On`, `Markup Color`, `Markup Font Size`, `Markup Position Start`, `Markup Radius`, `Markup Text`, `User Flag Set`, `Signal Flare`, `Smoke Trigger`, `Text To All`?**
  _High betweenness centrality (0.096) - this node is a cross-community bridge._
- **Why does `Context` connect `Core Event & Group Types` to `Async Shutdown & Streams`?**
  _High betweenness centrality (0.088) - this node is a cross-community bridge._
- **Why does `ShutdownHandle` connect `RPC Hook System` to `Server Error Handling`, `Core Event & Group Types`, `Lua Bridge & REPL`, `Async Shutdown & Streams`, `Stats & Metrics`?**
  _High betweenness centrality (0.049) - this node is a cross-community bridge._
- **What connects `dcs-grpc`, `dcs-grpc-repl`, `dcs-grpc-srs` to the rest of the system?**
  _13 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Server Admin & Coalition API` be split into smaller, more focused modules?**
  _Cohesion score 0.0515406162464986 - nodes in this community are weakly interconnected._
- **Should `Core Event & Group Types` be split into smaller, more focused modules?**
  _Cohesion score 0.051425213047311194 - nodes in this community are weakly interconnected._
- **Should `Unit Ammo & Description API` be split into smaller, more focused modules?**
  _Cohesion score 0.051189400782896716 - nodes in this community are weakly interconnected._