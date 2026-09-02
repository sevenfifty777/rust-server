---
name: dcs-grpc-client-development
description: Build, review, or debug Rust and Python clients for sevenfifty777/rust-server, including matching protobuf generation, API-key metadata, unary RPCs, MissionService streams, RecoveryService snapshots, and ownship hook mechanization. Do not use for implementing the Rust server or its DCS Lua bridge.
---

# DCS gRPC client development

Build clients against the contract and runtime behavior of the server version they will actually call. Do not infer an RPC name, message shape, enum value, stream item, or generated symbol.

## Establish the target contract

1. Identify the target server version, endpoint, authentication setting, and required services.
2. When the server checkout is available, treat its `protos/dcs/**/*.proto` files as the wire-contract authority. Use `protos/dcs/dcs.proto` as the umbrella import.
3. Search [references/dcs_grpc_api.md](references/dcs_grpc_api.md) for the exact service, RPC, request, response, enum, and nested message needed. Read only the relevant sections of this large generated reference.
4. If the target checkout differs from the bundled reference, regenerate bindings and documentation from the target protobufs. Never mix bindings from one release with a different server contract.
5. When behavior matters beyond the schema, inspect `src/server.rs`, `src/authentication.rs`, `src/rpc/**/*.rs`, and the corresponding `lua/DCS-gRPC/methods/*.lua`. A documented RPC can still depend on server configuration, mission state, the hook environment, SRS, or DCS objects.

The current server registers 19 versioned services under `dcs.<domain>.v0`. Package names are part of generated type and module paths; do not flatten them.

## Connect safely

- The server defaults to plaintext `http://127.0.0.1:50051`; `host` and `port` are configurable. Obtain the deployed endpoint from configuration instead of assuming the default. This server code does not configure native TLS.
- Authentication is optional. When enabled, every RPC is intercepted and must include `X-API-Key` metadata, including health checks and streams. Load the token from an environment variable or secrets facility; never hardcode, print, or log it.
- Use a short, explicit deadline for unary calls where appropriate. Do not put a whole-call deadline on an intentionally long-lived stream; provide cancellation and reconnect behavior instead.
- Check `MetadataService.GetHealth` and `GetVersion` during startup when useful, but do not treat a schema or compilation check as proof that a live DCS mission is ready.

## Rust clients

- Use `tokio` and versions of `tonic`, `tonic-build`, `prost`, and `prost-types` compatible with the selected server contract. This checkout uses `tonic`/`tonic-build` 0.13 and `prost` 0.13; do not silently upgrade an external project to unrelated latest versions.
- In this workspace, `dcs-grpc-stubs` generates bindings from `protos/dcs/dcs.proto`. Its default feature set creates neither clients nor servers; enable its `client` feature for client code. The generated path pattern is `dcs_grpc_stubs::<domain>::v0::<service>_service_client::<Service>ServiceClient`, unless the dependency is explicitly aliased.
- In an external project, either depend on matching generated bindings or compile the matching protobuf tree with client generation enabled. Preserve all package directories and imports.
- Model protobuf `optional` fields as `Option<T>`, repeated fields as `Vec<T>`, and `oneof` fields as generated enums. Match all known variants and handle an absent or future value without panicking.
- Attach `X-API-Key` through a request interceptor or per-request metadata. Mark sensitive metadata values as such when supported.
- Handle `tonic::Status` by code and retain useful context. Retry only transient transport failures and only when the operation is safe to repeat.

## Python clients

- Generate `_pb2.py` and `_pb2_grpc.py` modules from the matching protobuf files with mutually compatible `grpcio`, `grpcio-tools`, and protobuf-runtime versions. Compile all required service files and their imports; compiling only the import-only umbrella file does not create every Python service module.
- Prefer `grpc.aio` for concurrent clients and server-streaming RPCs. Use `grpc.aio.insecure_channel` only when connecting directly to this server's plaintext listener.
- Pass authentication metadata as `(("X-API-Key", token),)` on calls or through an interceptor.
- Consume streams with `async for`, catch `grpc.aio.AioRpcError`, inspect `code()` and `details()`, and reconnect with bounded exponential backoff plus cancellation support.
- Use `WhichOneof(...)` for protobuf `oneof` fields and handle no selected variant.

## Streaming behavior to preserve

- `MissionService.StreamEvents` is a long-running server stream and ends when the server or mission-side stream shuts down. Re-establish it after a transient disconnect.
- `MissionService.StreamUnits` defaults `poll_rate` to 5 seconds and `max_backoff` to 30 seconds. The server clamps `max_backoff` to at least `poll_rate`, sends an initial unit snapshot, then emits `unit` changes or `gone` notifications. Do not treat it as high-rate Tacview telemetry.
- Use `MissionService.GetSessionId` when continuity across mission reloads matters; the contract states that the ID changes on mission change or server restart.
- Treat stream timestamps according to their message documentation. Do not substitute wall-clock receipt time for mission-relative time.

## Recovery snapshots

Use `dcs.recovery.v0.RecoveryService.GetRecoverySnapshot` when a recovery-grading client needs the carrier transform, aircraft transform, and optional aircraft draw argument from one mission Lua callback. Prefer it over two independent `UnitService.GetTransform` calls for one logical observation.

- This is callback-atomic, not simulator-wide atomic: Lua captures one mission-relative `time`, then reads the two DCS objects sequentially inside the same callback. Do not claim both transforms came from an indivisible simulator state transaction.
- The request requires non-empty `carrier_name` and `aircraft_name`. Missing names produce `INVALID_ARGUMENT`; unresolved DCS units produce `NOT_FOUND`.
- `aircraft_draw_argument` is an optional `uint32`. Omit it when hook-state evidence is not needed; omission is semantically different from requesting argument zero.
- Interpret `aircraft_draw_argument.status` by its enum symbol: `NOT_REQUESTED`, `OBSERVED`, or `UNAVAILABLE`. Only use `value` when the status is `OBSERVED`; zero is a valid observed value. Preserve absent messages and unknown enum values defensively.
- `sequence` is generated by the client and echoed unchanged for correlation. It is not a server sample counter, timestamp, delivery guarantee, or proof that no observations were missed.
- The response transforms contain `Position`, `Orientation`, and `Velocity`. Rust bindings expose message-valued fields as `Option<T>`; validate required observations instead of unwrapping blindly.
- The Rust client path is `dcs_grpc_stubs::recovery::v0::recovery_service_client::RecoveryServiceClient`, unless the dependency is aliased. Python generated modules remain under the `dcs.recovery.v0` package hierarchy.
- A unary polling loop determines its own requested cadence, but actual sampling can be delayed by the client, transport, IPC queue, or Lua execution. Measure observed intervals and latency; do not call a timer setting or request loop a guaranteed sampling rate.

## Ownship hook mechanization

Use `dcs.hook.v0.HookService.GetOwnshipHookState` when a client running beside an occupied DCS cockpit needs the raw hook mechanization returned by `Export.LoGetMechInfo().hook`.

- This is a hook-environment, ownship-only observation. It is subject to DCS ownship-export permission and normally cannot describe AI aircraft, remote multiplayer aircraft, or an unoccupied dedicated server. `observation_status = UNAVAILABLE` is valid evidence of that limitation, not a hook-up indication.
- The response captures `model_time`, `aircraft_type`, optional `ownship_unit_id`, and optional raw `status_value`/`value`. Boolean DCS `hook.status` values are normalized to `0.0` or `1.0`; numeric values are preserved.
- Do not assume which raw field represents lever command, physical mechanism position, or up/down polarity. Establish those meanings with live before/after observations for every supported aircraft module before using them for grading or control decisions.
- Match `ownship_unit_id` to the unit being tracked before accepting an observation. Aircraft type alone is not a sufficient identity check. Treat an absent or mismatched ID as diagnostic-only evidence.
- This RPC is separate from `RecoveryService.GetRecoverySnapshot`: it does not make hook-environment data callback-atomic with mission-environment transforms. Timestamp and correlate the two streams explicitly.
- The Rust client path is `dcs_grpc_stubs::hook::v0::hook_service_client::HookServiceClient`. Handle the `OwnshipHookObservationStatus` enum defensively and preserve absent optional values.

## Status and retry policy

The current implementation can expose these important codes:

- `UNAUTHENTICATED`: missing or invalid `X-API-Key` while authentication is enabled.
- `PERMISSION_DENIED`: notably, `Eval` is disabled by default unless `evalEnabled` is configured.
- `INVALID_ARGUMENT`, `NOT_FOUND`, `ALREADY_EXISTS`, and `UNIMPLEMENTED`: mapped from supported Lua bridge errors.
- `INTERNAL`: unexpected Lua/IPC failures and server-side conversion errors.
- `UNAVAILABLE`: transport interruption or a service dependency such as SRS being unavailable.

Do not retry authentication, validation, permission, or not-found failures blindly. For transient connection failures, use bounded exponential backoff with jitter and reset the backoff after a stable connection. Consider idempotency before retrying state-changing RPCs.

## Verification

- Compile or type-check generated client code against the selected protobuf version.
- Test metadata injection, one representative unary call, stream cancellation/reconnect, one important `oneof`, and failure handling. Use an isolated test server or mocks unless the user explicitly wants a live DCS test.
- For a live check, report the endpoint, server version from `GetVersion`, exact RPCs exercised, and observed status codes without exposing credentials.
- Never claim live mission, DCS object, SRS, or stream behavior was validated when only source inspection or compilation was performed.
