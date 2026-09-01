# Vendored dcs-module-ipc

This crate is based on `dcs-module-ipc` 0.9.1 from
<https://github.com/rkusa/dcs-module-ipc>, crates.io checksum
`77f7fd9be53b6f71edc7c8ad74004d532078d4ca9ef7bda768280ca271414b5d` and
upstream commit `7a647441abd6d81b0637b636cc9ddc5b9e072ed6`.

It remains licensed under `MIT OR Apache-2.0`. The local copy adds bounded
queues, cancellation-aware dequeue, request correlation metadata, and queue
timing needed by DCS-gRPC recovery telemetry. Keeping it as a workspace crate
makes those changes reproducible and testable with the server.
