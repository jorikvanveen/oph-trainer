#!/usr/bin/env bash
set -euo pipefail

cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../backend"
exec cargo test --locked --package oph-trainer-backend export_bindings "$@"
