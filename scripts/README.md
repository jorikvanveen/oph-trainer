# Project scripts

Run these scripts in the flake devshell. They resolve paths relative to their own location, so they work from any directory.

Install frontend dependencies once with `npm ci --prefix frontend` from the repository root.

| Script | Task |
| --- | --- |
| `dev-backend.sh` | Run the backend with Cargo. Extra arguments go to `cargo run`. |
| `dev-frontend.sh` | Run the Vite development server. Extra arguments go to Vite. |
| `check.sh` | Check Rust formatting, run workspace Clippy with warnings denied, and check Svelte/TypeScript. |
| `export-dtos.sh` | Regenerate TypeScript DTOs in `frontend/src/lib/dto/` using the ts-rs export tests. |
| `build.sh` | Export DTOs, build the static frontend, and compile the release backend. |
| `start.sh` | Run the release backend. Run `build.sh` first. |
| `migrate.sh` | Run SeaORM migration commands. With no arguments, show migration status. |

For development, run these in separate terminals:

```sh
./scripts/dev-backend.sh
./scripts/dev-frontend.sh
```

Vite forwards `/api` to `http://127.0.0.1:3000`, the default backend address.

To build and serve the app without Vite:

```sh
./scripts/build.sh
./scripts/start.sh
```

The backend serves the app at `http://127.0.0.1:3000` by default. The scripts leave `OPH_BIND_ADDRESS`, `OPH_DATABASE_URL`, `OPH_STATIC_DIR`, and `RUST_LOG` unchanged. Relative backend paths resolve from `backend/`.

## Migrations

```sh
./scripts/migrate.sh
./scripts/migrate.sh generate create_accounts
./scripts/migrate.sh up
```

`migrate.sh` uses `OPH_DATABASE_URL`, or `sqlite://oph-trainer.db?mode=rwc` when unset. This points to the same database as the backend. The backend also applies pending migrations on startup.

`down`, `reset`, `refresh`, and `fresh` can remove tables and data. Check the target database before using them.

The scripts do not read or source `.env` files.
