# amq-protocol

[![API Docs](https://docs.rs/amq-protocol/badge.svg)](https://docs.rs/amq-protocol)
[![Build status](https://github.com/amqp-rs/amq-protocol/workflows/Build%20and%20test/badge.svg)](https://github.com/amqp-rs/amq-protocol/actions)
[![Downloads](https://img.shields.io/crates/d/amq-protocol.svg)](https://crates.io/crates/amq-protocol)

## Features

- codegen: force code generation (default to pregenerated sources)
- hickory-dns: use hickory-dns for domain name resolution to avoid spurious network hangs
- vendored-openssl: use a vendored openssl version instead of the system one (when using openssl backend)
- verbose-errors: enable more verbose errors in the AMQP parser

## TLS backends

- native-tls
- openssl
- rustls (default)

## Rustls certificates store

- rustls-native-certs (default)
- rustls-webpki-roots-certs

## Warning about crypto backends for rustls

A crypto implementation must be enabled in rustls using feature flags.
We mimic what rustls does, providing one feature flag per implementation and enabling the same as rustls by default.
Available options are:
- `rustls--aws_lc_rs` (default)
- `rustls--ring`

