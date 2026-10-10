#!/usr/bin/env bash
set -euo pipefail

cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.."

entity_workspace=$(mktemp -d "${TMPDIR:-/tmp}/oph-trainer-entities-XXXXXX")
trap 'rm -rf -- "$entity_workspace"' EXIT

entity_database_url="sqlite://$entity_workspace/schema.sqlite?mode=rwc"
OPH_DATABASE_URL="$entity_database_url" ./scripts/migrate.sh up

sea-orm-cli generate entity \
    --database-url "$entity_database_url" \
    --output-dir backend/src/entity \
    --compact-format \
    --with-prelude none \
    --with-serde none \
    --big-integer-type i64
