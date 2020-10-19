# bazel-example

Builds a deterministic container image including grpc-health-check with [Bazel](https://bazel.build/) from a
managed Google-maintained base image.

Managed base images are base container images that are automatically patched by Google for security vulnerabilities, 
using the most recent patches available from the project upstream.

[Learn more](https://cloud.google.com/container-registry/docs/managed-base-images)