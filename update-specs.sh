#!/usr/bin/env bash

main() {
    cd codegen/specs
    curl -O https://raw.githubusercontent.com/rabbitmq/rabbitmq-codegen/master/amqp-rabbitmq-0.9.1.json
}

main "${@}"
