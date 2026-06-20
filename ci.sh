#!/usr/bin/env sh
set -e

echo "==> build"
cargo build --workspace

echo "==> test"
cargo test --workspace

echo "==> clippy"
cargo clippy --workspace -- -D warnings

echo "==> fmt"
cargo fmt --all -- --check

echo ""
echo "all checks passed"
