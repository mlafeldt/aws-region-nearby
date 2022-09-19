# aws-region-nearby

[![Latest version](https://img.shields.io/crates/v/aws-region-nearby.svg)](https://crates.io/crates/aws-region-nearby)
[![Documentation](https://docs.rs/aws-region-nearby/badge.svg)](https://docs.rs/aws-region-nearby)
[![CI](https://github.com/mlafeldt/aws-region-nearby/workflows/CI/badge.svg)](https://github.com/mlafeldt/aws-region-nearby/actions)

A Rust library to find the nearest AWS region to a given location.

Especially useful when you run code at the edge and want fast access to regional AWS services, e.g. [Cloudflare Workers accessing DynamoDB global tables](https://artofserverless.com/aws-region-nearby/).

Check out the [examples](examples) to learn how to use aws-region-nearby with different AWS SDKs, Cloudflare Workers, and Deno Deploy. The included [infra code](examples/infra) will deploy a replicated DynamoDB table for testing.

Many thanks to [@tobilg](https://github.com/tobilg) for providing the original AWS region coordinates.

## Quickstart

Add the crate as a dependency to your `Cargo.toml`:

```toml
[dependencies]
aws-region-nearby = "0.2"
```

Now you can start [finding AWS regions](https://docs.rs/aws-region-nearby).

## License

Copyright (c) 2021-2022 Mathias Lafeldt

Licensed under the [Apache License, Version 2.0](LICENSE-APACHE) or the [MIT license](LICENSE-MIT), at your option.
