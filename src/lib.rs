//! A Rust library to find the nearest AWS region to a given location.
//!
//! Especially useful when you run code at the edge and want fast access to
//! regional AWS services, e.g. Cloudflare Workers accessing DynamoDB global
//! tables.

#![deny(clippy::all, clippy::nursery)]
#![deny(nonstandard_style, rust_2018_idioms)]
#![deny(missing_docs, missing_debug_implementations)]

mod error;
pub use error::Error;

mod aws;
pub use aws::*;
