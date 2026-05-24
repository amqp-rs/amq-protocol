<div align="center">

[![API Docs](https://docs.rs/amq-protocol/badge.svg)](https://docs.rs/amq-protocol)
[![Build status](https://github.com/amqp-rs/amq-protocol/workflows/Build%20and%20test/badge.svg)](https://github.com/amqp-rs/amq-protocol/actions)
[![Downloads](https://img.shields.io/crates/d/amq-protocol.svg)](https://crates.io/crates/amq-protocol)
[![Dependency Status](https://deps.rs/repo/github/amqp-rs/amq-protocol/status.svg)](https://deps.rs/repo/github/amqp-rs/amq-protocol)
[![LICENSE](https://img.shields.io/crates/l/amq-protocol)](LICENSE)

**AMQP 0-9-1 protocol codec and TCP/TLS connection layer for Rust.**

</div>

This workspace provides the low-level AMQP 0-9-1 building blocks used by
[lapin](https://docs.rs/lapin). It is split into focused sub-crates that can
also be used independently.

## Crates in this workspace

| Crate | docs.rs | Purpose |
|-------|---------|---------|
| [`amq-protocol`](protocol/) | [![docs](https://docs.rs/amq-protocol/badge.svg)](https://docs.rs/amq-protocol) | Main entry point; re-exports all sub-crates |
| [`amq-protocol-types`](types/) | [![docs](https://docs.rs/amq-protocol-types/badge.svg)](https://docs.rs/amq-protocol-types) | AMQP scalar and compound types, wire-format codec |
| [`amq-protocol-uri`](uri/) | [![docs](https://docs.rs/amq-protocol-uri/badge.svg)](https://docs.rs/amq-protocol-uri) | `amqp://` / `amqps://` URI parsing |
| [`amq-protocol-tcp`](tcp/) | [![docs](https://docs.rs/amq-protocol-tcp/badge.svg)](https://docs.rs/amq-protocol-tcp) | TCP/TLS connection from an `AMQPUri` |
| [`amq-protocol-codegen`](codegen/) | [![docs](https://docs.rs/amq-protocol-codegen/badge.svg)](https://docs.rs/amq-protocol-codegen) | Build-time code generation from the RabbitMQ spec |

Most users should depend on `amq-protocol` directly — it re-exports everything
and wires up the sub-crates. Depend on a sub-crate only when you need a strict
subset of the functionality.

## Feature flags

### Async runtime (pick exactly one)

| Flag | Notes |
|------|-------|
| `tokio` *(default)* | Requires a running Tokio runtime |
| `smol` | Uses the smol executor |
| `async-global-executor` | Uses async-global-executor |

### TLS backend (pick at most one)

| Flag | Notes |
|------|-------|
| `rustls` *(default)* | TLS via rustls |
| `native-tls` | TLS via the platform's native library |
| `openssl` | TLS via OpenSSL |

### Rustls certificate store (when `rustls` is active)

| Flag | Notes |
|------|-------|
| `rustls-platform-verifier` *(default)* | Uses the platform trust store |
| `rustls-native-certs` | Loads native root certificates |
| `rustls-webpki-roots-certs` | Uses the webpki bundled root set |

### Rustls crypto provider (at least one required)

| Flag | Notes |
|------|-------|
| `rustls--aws_lc_rs` *(default)* | Uses aws-lc-rs |
| `rustls--ring` | Uses ring (more portable, e.g. builds on Windows) |

### Miscellaneous

| Flag | Notes |
|------|-------|
| `codegen` | Force protocol code regeneration at build time |
| `verbose-errors` | More detailed AMQP parser error messages |
| `hickory-dns` | Hickory DNS resolver (avoids spurious network hangs) |
