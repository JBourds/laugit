#!/bin/bash
export LOGOS_EVENTS=TestEvent
export LOGOS_LEVEL=Debug

# Run tests with std feature
echo "Running std tests..."
cargo test --features std -- --nocapture
if [ $? -ne 0 ]; then
    echo "Tests with std feature failed."
    exit 1
fi

# Run tests without std feature
echo "Running no_std tests..."
cargo test -- --nocapture
if [ $? -ne 0 ]; then
    echo "Tests without std feature failed."
    exit 1
fi
