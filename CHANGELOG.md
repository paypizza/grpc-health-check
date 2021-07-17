<a name="0.1.1"></a>
## [0.1.1](https://github.com/paypizza/grpc-health-check/compare/0.1.0...0.1.1) (2021-06-17)

### Features

- use mimalloc as a global allocator for musl libc ([e28e426](https://github.com/paypizza/grpc-health-check/commit/e28e426ea693f4aae7b84d33c935e90b877c3526))

### Bug Fixes

- read the tls root certificate file synchronously ([85f359d](https://github.com/paypizza/grpc-health-check/commit/85f359db06945f99c045706942062804f314a26f))
- reword the tls client error messages ([53396f4](https://github.com/paypizza/grpc-health-check/commit/53396f4ea6cf186cc1e4d9f4a78f8d4267091961))
- use `localhost` instead of `localhost` as the default address ([747b97ea](https://github.com/paypizza/grpc-health-check/commit/747b97ea2716fa2558e7f911c60e2e2c1e0e74d7)), closes [#2](https://github.com/paypizza/grpc-health-check/issues/2)

<a name="0.1.0"></a>
## 0.1.0 (2020-10-18)

Initial release