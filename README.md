# bazel-rust-docker-cross-example

Just a small example to demonstrate how to cross-compile rust with bazel and zig-cc.

_it has been tested on a M1, for cross-compilation from MacOS to Linux gnu targets_

This project uses:

- bazelisk
- bazel 6
- cargo universe
- zig-cc
- rules_docker

---

```bash
> bazel run //service:image
INFO: Analyzed target //service:image (109 packages loaded, 28612 targets configured).
INFO: Found 1 target...
Target //service:image up-to-date:
  bazel-bin/service/image-layer.tar
INFO: Elapsed time: 64.218s, Critical Path: 54.24s
INFO: 100 processes: 24 internal, 76 darwin-sandbox.
INFO: Build completed successfully, 100 total actions
INFO: Build completed successfully, 100 total actions
9392bb825a55: Loading layer [==================================================>]  1.823MB/1.823MB
Loaded image ID: sha256:3c94013c66c90589d6ef32baa6c00b409d8f77e10cce0adeba303c1f4ed8bd25
Tagging 3c94013c66c90589d6ef32baa6c00b409d8f77e10cce0adeba303c1f4ed8bd25 as bazel/service:image
WARNING: The requested image's platform (linux/amd64) does not match the detected host platform (linux/arm64/v8) and no specific platform was requested
got 0
got 1
got 2

```

The flags are specificied in the `.bazelrc`

```bash
> cat .bazelrc
build --incompatible_enable_cc_toolchain_resolution # required for bazel-zig-cc
build --extra_toolchains @zig_sdk//toolchain:x86_64-linux-gnu.2.31 # the linux toolchain
build --experimental_strict_action_env # required to avoid the `sed: command not found` issue
```
