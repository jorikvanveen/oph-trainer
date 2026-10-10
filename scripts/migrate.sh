#!/usr/bin/env bash
set -euo pipefail

cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../backend"

if [[ $# -eq 0 ]]; then
    set -- status
fi

exec sea-orm-cli migrate -d migration \
    -u "${OPH_DATABASE_URL-sqlite://oph-trainer.db?mode=rwc}" "$@"
