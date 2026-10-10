#!/usr/bin/env bash
set -euo pipefail

cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../backend"
exec cargo run --locked --package oph-trainer-backend "$@"
