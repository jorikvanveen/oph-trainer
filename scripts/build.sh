#!/usr/bin/env bash
set -euo pipefail

cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.."
./scripts/export-dtos.sh

cd frontend
npm run build

cd ../backend
cargo build --locked --release --package oph-trainer-backend
