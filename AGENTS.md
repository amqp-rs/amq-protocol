# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Build / check
cargo check --all --bins --examples --tests --all-features

# Test
cargo test

# Lint
cargo clippy --all-features -- -W clippy::all

# Format check / apply
cargo fmt --all -- --check
cargo fmt --all

# Force-regenerate protocol.rs from the AMQP spec
cargo build -p amq-protocol --features codegen
```

Minimum supported Rust version: **1.88.0** (edition 2024).

## Workspace Structure

Five crates, one public API surface:

| Crate | Path | Role |
|---|---|---|
| `amq-protocol` | `protocol/` | Public API; re-exports `types`, `uri`, `tcp`; runs codegen |
| `amq-protocol-types` | `types/` | AMQP primitive types, serialization (cookie-factory), parsing (nom) |
| `amq-protocol-uri` | `uri/` | AMQP URI parsing |
| `amq-protocol-tcp` | `tcp/` | TCP/TLS async connection layer |
| `amq-protocol-codegen` | `codegen/` | Build-time code generator (library only, never a binary) |

`amq-protocol` is the entry point for consumers; the sub-crates are also published individually for fine-grained use.

## Code Generation Pipeline

Protocol code is generated at build time from the embedded RabbitMQ spec:

1. **Spec:** `codegen/specs/amqp-rabbitmq-0.9.1.json`
2. **Generator:** `codegen/src/specs.rs` loads and normalises the spec (`internal.rs` applies RabbitMQ-specific overrides)
3. **Template:** `protocol/templates/protocol.rs` (Handlebars)
4. **Build script:** `protocol/build.rs` writes the result to `OUT_DIR/protocol.rs`
5. **Fallback:** `protocol/src/generated.rs` is a pre-generated copy checked into the repo; it is used when the `codegen` feature is disabled

To update the generated code after changing the template or spec, run `cargo build -p amq-protocol --features codegen` and commit `protocol/src/generated.rs`.

## Feature Flags

`amq-protocol` and `amq-protocol-tcp` expose feature flags that select async runtime, TLS backend, and crypto provider:

- **Async runtime** (mutually exclusive): `tokio` (default), `smol`, `async-global-executor`
- **TLS backend** (mutually exclusive): `rustls` (default), `native-tls`, `openssl`
- **rustls crypto provider** (at least one required): `rustls--aws_lc_rs` (default), `rustls--ring`
- **TLS cert store** (for rustls): `rustls-platform-verifier` (default, enabled transitively by `rustls`), `rustls-native-certs`, `rustls-webpki-roots-certs`
- **DNS:** `hickory-dns`
- **Parser errors:** `verbose-errors`
- **Codegen:** `codegen` / `codegen-internal`

When adding dependencies to `amq-protocol-tcp`, verify they work across all supported runtimes and TLS combinations.

## Key Design Patterns

- **Parsing** uses `nom` combinators; parsers live in `types/src/parsing.rs` and `protocol/src/frame/parsing.rs`.
- **Serialization** uses `cookie-factory`; generators live in `types/src/generation.rs` and `protocol/src/frame/generation.rs`.
- **Authentication** (`protocol/src/auth.rs`) implements SASL mechanisms: `AMQPlain`, `Plain`, `External`, `Anonymous`, `RabbitCrDemo`.
- AMQP type aliases (`ChannelId`, `DeliveryTag`, etc.) are defined in `types/src/types.rs` and re-exported through `amq-protocol`.
