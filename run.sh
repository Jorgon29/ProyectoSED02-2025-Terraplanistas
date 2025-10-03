#!/bin/bash

set -e

set -a
source .env
set +a

if [ -z "$1" ]; then
    echo "Usage: $0 [d|p]"
    echo "  d: Run development build (cargo run)"
    echo "  p: Run production build (cargo run --release)"
    exit 1
fi

MODE=$1

if [ "$MODE" = "d" ]; then
    echo "Running in DEVELOPMENT mode..."
    cargo run
elif [ "$MODE" = "p" ]; then
    echo "Running in PRODUCTION mode (Release build)..."
    cargo run --release
else
    echo "Invalid argument: $MODE. Use 'd' for development or 'p' for production."
    exit 1
fi