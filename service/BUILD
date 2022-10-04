load("@rules_rust//rust:defs.bzl", "rust_binary")
load("@crate_index_cargo_workspace//:defs.bzl", "all_crate_deps")
load("@io_bazel_rules_docker//rust:image.bzl", "rust_image")

rust_binary(
    name = "bin",
    srcs = [
        "src/main.rs",
    ],
    edition = "2021",
    proc_macro_deps = all_crate_deps(proc_macro = True),
    deps = all_crate_deps(),
)

rust_image(
    name = "image",
    binary = ":bin",
)
