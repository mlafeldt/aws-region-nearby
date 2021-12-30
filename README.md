# aws-region-nearby

[![Latest version](https://img.shields.io/crates/v/aws-region-nearby.svg)](https://crates.io/crates/aws-region-nearby)
[![Documentation](https://docs.rs/aws-region-nearby/badge.svg)](https://docs.rs/aws-region-nearby)
[![CI](https://github.com/mlafeldt/aws-region-nearby/workflows/Rust/badge.svg)](https://github.com/mlafeldt/aws-region-nearby/actions)

A Rust library to find the nearest AWS region to a given location.

Many thanks to [@tobilg](https://gist.github.com/tobilg) for providing the [AWS region coordinates](src/lib.rs).

## Usage

Use with [AWS SDK for Rust](https://github.com/awslabs/aws-sdk-rust):

```rust
let region = aws_region_nearby::find_region_nearby(latitude, longitude);
let sdk_region = aws_types::region::Region::from_static(region.name());
```

Use with [Rusoto](https://github.com/rusoto/rusoto):

```rust
use std::str::FromStr;

let region = aws_region_nearby::find_region_nearby(latitude, longitude);
let rusoto_region = rusoto_core::Region::from_str(region.name()).unwrap();
```

## License

Copyright (c) 2021 Mathias Lafeldt

Licensed under the [Apache License, Version 2.0](LICENSE-APACHE) or the [MIT license](LICENSE-MIT), at your option.
