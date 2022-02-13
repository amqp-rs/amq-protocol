#!/usr/bin/env bash

main() {
    cd codegen/specs
    curl -O https://raw.githubusercontent.com/rabbitmq/rabbitmq-server/master/deps/rabbitmq_codegen/amqp-rabbitmq-0.9.1.json
}

main "${@}"
