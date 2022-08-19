workspace(name = "paypizza_grpc_health_check")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "bazel_skylib",
    sha256 = "f7be3474d42aae265405a592bb7da8e171919d74c16f082a5457840f06054728",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/bazel-skylib/releases/download/1.2.1/bazel-skylib-1.2.1.tar.gz",
        "https://github.com/bazelbuild/bazel-skylib/releases/download/1.2.1/bazel-skylib-1.2.1.tar.gz",
    ],
)

load("@bazel_skylib//:workspace.bzl", "bazel_skylib_workspace")

bazel_skylib_workspace()

http_archive(
    name = "rules_proto",
    sha256 = "e017528fd1c91c5a33f15493e3a398181a9e821a804eb7ff5acdd1d2d6c2b18d",
    strip_prefix = "rules_proto-4.0.0-3.20.0",
    urls = ["https://github.com/bazelbuild/rules_proto/archive/refs/tags/4.0.0-3.20.0.tar.gz"],
)

load(
    "@rules_proto//proto:repositories.bzl",
    "rules_proto_dependencies",
    "rules_proto_toolchains",
)

rules_proto_dependencies()

rules_proto_toolchains()

http_archive(
    name = "rules_rust",
    sha256 = "2f3a8181ca9d5dc2750de9a4b117539e2184c18ec40f9b6304c53df904539ffa",
    strip_prefix = "rules_rust-51c0658415b1e31ec21dac1207a09cfe4630fc73",
    urls = [
        "https://github.com/bazelbuild/rules_rust/archive/51c0658415b1e31ec21dac1207a09cfe4630fc73.tar.gz",
    ],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains(
    edition = "2021",
    version = "1.63.0",
)

load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")

crate_universe_dependencies(bootstrap = True)

load("//crates:crates.bzl", "crate_repositories")

crate_repositories()