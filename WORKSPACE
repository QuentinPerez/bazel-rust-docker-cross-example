load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# RUST #########################################################################

http_archive(
    name = "rules_rust",
    sha256 = "0cc7e6b39e492710b819e00d48f2210ae626b717a3ab96e048c43ab57e61d204",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/rules_rust/releases/download/0.10.0/rules_rust-v0.10.0.tar.gz",
        "https://github.com/bazelbuild/rules_rust/releases/download/0.10.0/rules_rust-v0.10.0.tar.gz",
    ],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains(
    edition = "2021",
    extra_target_triples = [
        # "aarch64-apple-ios-sim",
        # "aarch64-apple-ios",
        # "aarch64-linux-android",
        # "x86_64-unknown-linux-gnu",
        # "armv7-linux-androideabi",
        # "i686-linux-android",
        # "x86_64-linux-android",
    ],
    # rustup default nightly-2022-09-10
    include_rustc_srcs = True,
    iso_date = "2022-09-10",
    rustfmt_version = "nightly",
    version = "nightly",
)

load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")

crate_universe_dependencies()

load("@rules_rust//crate_universe:defs.bzl", "crates_repository")

crates_repository(
    name = "crate_index_cargo_workspace",
    cargo_lockfile = "//:Cargo.Bazel.lock",
    lockfile = "//:cargo-bazel-lock.json",
    manifests = [
        "//:Cargo.toml",
        "//service:Cargo.toml",
    ],
)

load(
    "@crate_index_cargo_workspace//:defs.bzl",
    cargo_workspace_crate_repositories = "crate_repositories",
)

cargo_workspace_crate_repositories()

# Docker #######################################################################

# http_archive(
#     name = "io_bazel_rules_go",
#     sha256 = "099a9fb96a376ccbbb7d291ed4ecbdfd42f6bc822ab77ae6f1b5cb9e914e94fa",
#     urls = [
#         "https://mirror.bazel.build/github.com/bazelbuild/rules_go/releases/download/v0.35.0/rules_go-v0.35.0.zip",
#         "https://github.com/bazelbuild/rules_go/releases/download/v0.35.0/rules_go-v0.35.0.zip",
#     ],
# )

# load("@io_bazel_rules_go//go:deps.bzl", "go_register_toolchains", "go_rules_dependencies")

# go_rules_dependencies()

# go_register_toolchains(version = "1.19.1")

http_archive(
    name = "io_bazel_rules_docker",
    sha256 = "b1e80761a8a8243d03ebca8845e9cc1ba6c82ce7c5179ce2b295cd36f7e394bf",
    urls = ["https://github.com/bazelbuild/rules_docker/releases/download/v0.25.0/rules_docker-v0.25.0.tar.gz"],
)

load(
    "@io_bazel_rules_docker//repositories:repositories.bzl",
    container_repositories = "repositories",
)

container_repositories()

load("@io_bazel_rules_docker//repositories:deps.bzl", container_deps = "deps")

container_deps()

load(
    "@io_bazel_rules_docker//rust:image.bzl",
    _rust_image_repos = "repositories",
)

_rust_image_repos()

# load("@io_bazel_rules_docker//container:container.bzl", "container_pull")

# container_pull(
#     name = "rust_base_image",
#     registry = "gcr.io",
#     repository = "distroless/static-debian11",
#     tag = "latest",
# )

# BAZEL_ZIG_CC_VERSION = "v1.0.0-rc3"

# http_archive(
#     name = "bazel-zig-cc",
#     sha256 = "a309e20189d285a2185d69121b5dbe0db4a5b32e635a5add17d0d380c142585b",
#     strip_prefix = "bazel-zig-cc-{}".format(BAZEL_ZIG_CC_VERSION),
#     urls = ["https://git.sr.ht/~motiejus/bazel-zig-cc/archive/{}.tar.gz".format(BAZEL_ZIG_CC_VERSION)],
# )

# load("@bazel-zig-cc//toolchain:defs.bzl", zig_toolchains = "toolchains")

# version, url_formats and host_platform_sha256 are optional, but highly
# recommended. Zig SDK is by default downloaded from dl.jakstys.lt, which is a
# tiny server in the closet of Yours Truly.
# zig_toolchains()
# zig_toolchains(
#     version = "<...>",
#     url_formats = [
#         "https://example.org/zig/zig-{host_platform}-{version}.{_ext}",
#     ],
#     host_platform_sha256 = { ... },
# )
