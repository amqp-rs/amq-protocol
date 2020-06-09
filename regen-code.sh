#!/usr/bin/env bash

main() {
    export AMQ_PROTOCOL_CODEGEN_DIR="$(dirname "${0}" | xargs realpath)/protocol/src/"
    export AMQ_PROTOCOL_CODEGEN_FILE="generated"

    cargo build --manifest-path protocol/Cargo.toml --features=codegen-internal
    rustfmt "${AMQ_PROTOCOL_CODEGEN_DIR}/${AMQ_PROTOCOL_CODEGEN_FILE}.rs"
}

main "${@}"
