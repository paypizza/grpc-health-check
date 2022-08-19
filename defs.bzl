# buildifier: disable=module-docstring
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:utils.bzl", "maybe")
load("//crates:defs.bzl", _crate_repositories = "crate_repositories")

def paypizza_grpc_health_check_repositories():
    """Repositories used in the implementation of `paypizza_grpc_health_check`."""

    maybe(
        http_archive,
        name = "rules_rust",
        urls = ["https://github.com/bazelbuild/rules_rust/archive/51c0658415b1e31ec21dac1207a09cfe4630fc73.tar.gz"],
        sha256 = "2f3a8181ca9d5dc2750de9a4b117539e2184c18ec40f9b6304c53df904539ffa",
        strip_prefix = "rules_rust-51c0658415b1e31ec21dac1207a09cfe4630fc73",
    )

    maybe(
        http_archive,
        name = "rules_proto",
        urls = ["https://github.com/bazelbuild/rules_proto/archive/refs/tags/4.0.0-3.20.0.tar.gz"],
        sha256 = "e017528fd1c91c5a33f15493e3a398181a9e821a804eb7ff5acdd1d2d6c2b18d",
        strip_prefix = "rules_proto-4.0.0-3.20.0",
    )

paypizza_grpc_health_check_crate_repositories = _crate_repositories