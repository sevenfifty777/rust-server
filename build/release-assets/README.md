# Release assets

This directory contains version-neutral files that cannot be generated from the Rust or Lua source tree during a
release build.

`Missions/DCS-gRPC-Example.miz` is the packaged example mission. Its gRPC-specific action calls `GRPC.load()` and
does not embed a DCS-gRPC release number. Update and validate it in the DCS Mission Editor when the example itself
needs to change.

Do not source current packages from `sample_release/DCS-gRPC-0.8.1`. That ignored directory is retained only as a
historical release archive. The release script copies current protobufs from `protos/dcs`, builds `repl.exe` from
the current checkout, and does not bundle third-party `grpcurl` or `grpcui` executables.
Official download links for those optional clients are packaged from
`Tools/DCS-gRPC/OPTIONAL-TOOLS.txt` instead.
