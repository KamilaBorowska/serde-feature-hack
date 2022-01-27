//! **DEPRECATED**: Since Rust 1.31 it's possible to
//! [rename dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#renaming-dependencies-in-cargotoml).
//! There is no need to use this crate anymore.
//!
//! A hack to allow having a feature named `serde` which doesn't just depend on serde
//!
//! In Cargo.toml, you can do the following:
//!
//! ```toml
//! [dependencies]
//! serde-feature-hack = { version = "0.1.0", optional = true }
//!
//! [features]
//! serde = ["serde-feature-hack", "some-other-dependency"]
//! ```
//!
//! Then, you can use `serde` like you normally would.
//!
//! ```
//! extern crate serde;
//! extern crate serde_json;
//!
//! use serde::{Serialize, Serializer};
//!
//! struct X;
//!
//! impl Serialize for X {
//!     fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
//!         serializer.serialize_str("Hello, world!")
//!     }
//! }
//!
//! assert_eq!(serde_json::to_string(&X).unwrap(), r#""Hello, world!""#);
//! ```

#![no_std]

pub use real_serde::*;
