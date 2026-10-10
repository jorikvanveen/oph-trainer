#!/usr/bin/env bash
set -euo pipefail

cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../backend"
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets -- -D warnings

cd ../frontend
npm run check
