# authorization-server
Authorization Server with Rust using Tonic.

## Function implemented

- User registration and profile store
- Change password
- Login
- Token authentication
- Get and automatically refreshes Token

##### Crates Used

- [tonic](https://crates.io/crates/tonic) // A gRPC over HTTP/2 implementation focused on high performance, interoperability, and flexibility. 
- [tonic-build](https://crates.io/crates/tonic-build) // Codegen module of `tonic` gRPC implementation.
- [tokio](https://crates.io/crates/tokio) // An event-driven, non-blocking I/O platform for writing asynchronous I/O backed applications. 
- [rust-argon2](https://crates.io/crates/rust-argon2) // crate for hashing passwords using the cryptographically-secure Argon2 hashing algorithm.
- [chrono](https://crates.io/crates/chrono) // Date and time library for Rust.
- [diesel](https://crates.io/crates/diesel) // A safe, extensible ORM and Query Builder for PostgreSQL, SQLite, and MySQL.
- [dotenv](https://crates.io/crates/dotenv) // A dotenv implementation for Rust.
- [derive_more](https://crates.io/crates/derive_more) // Convenience macros to derive tarits easily
- [env_logger](https://crates.io/crates/env_logger) // A logging implementation for log which is configured via an environment variable.
- [once_cell](https://crates.io/crates/once_cell) // Single assignment cells and lazy values.
- [thiserror](https://crates.io/crates/thiserror) // This library provides a convenient derive macro for the standard library's std::error::Error trait.
- [serde](https://crates.io/crates/serde) // A generic serialization/deserialization framework.
- [serde_json](https://crates.io/crates/serde_json) // A JSON serialization file format.
- [config](https://crates.io/crates/config) // Layered configuration system for Rust applications.
- [tracing](https://crates.io/crates/tracing) // Application-level tracing for Rust.
- [tracing-subscriber](https://crates.io/crates/tracing-subscriber) // Utilities for implementing and composing `tracing` subscribers. 
- [rand](https://crates.io/crates/rand) // Random number generators and other randomness functionality. 
- [redis](https://crates.io/crates/redis) // Redis driver for Rust.
- [jsonwebtoken](https://crates.io/crates/jsonwebtoken) // Create and decode JWTs in a strongly typed way.
- [prost](https://crates.io/crates/prost) // A Protocol Buffers implementation for the Rust Language.
- [prost-derive](https://crates.io/crates/prost-derive) // prost-derive handles generating encoding and decoding implementations for Rust types annotated with prost annotation.

## TODO

- Request cache
- Error stack trace
- Pg support
- Middleware
- etc
