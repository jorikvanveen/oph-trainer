# Project scripts

Prefer the scripts in `scripts/` for common development tasks.

Run direct Cargo check, build, test, and DTO export commands with `backend/` as the working directory, for example `cargo test --workspace`. Cargo discovers `.cargo/config.toml` from the working directory, not the `--manifest-path` location. Running tests from the repository root skips `backend/.cargo/config.toml` and makes ts-rs write generated files to `backend/bindings/` instead of `frontend/src/lib/dto/`.

The flake devshell is already loaded into the agent's environment. Run scripts and tool commands directly; do not wrap them in `nix develop`.

Before running the app, checking or building changes, exporting DTOs, or managing migrations, read `scripts/README.md` for the available commands and usage.

# Formatting

After editing frontend files, format them with Prettier and `prettier-plugin-svelte`: `npm run format --prefix frontend -- <paths>`, with paths relative to `frontend/`. The formatter configuration excludes generated DTOs and build outputs.

After editing Rust, run `cargo fmt --manifest-path backend/Cargo.toml --all` from the repository root.

# Database access

Use SeaORM entities and query builders for database operations, and SeaORM migration builders for schema changes. Never use raw SQL, including in migrations, queries, or tests.

Use the SeaORM CLI wherever it supports the task. Scaffold migrations with `scripts/migrate.sh generate <name>`, then fill in their builders. Generate entities from the migrated schema with `scripts/generate-entities.sh`; do not handwrite entity models or relations. Read `scripts/README.md` for this workflow before changing the schema or entities.

Before changing account persistence or authentication, read `docs/adr/0002-account-keys-as-sole-credentials.md`. Account keys are stored directly as the primary key and sent in `Authorization: Bearer` headers.

# HTTP request handling

Use Axum's standard extractors and built-in rejection responses for request parsing and deserialization. Keep custom API errors for domain validation. Replace extractor rejection handling only when the user explicitly requests it.

# Comments

Use comments only to explain non-obvious details or reasoning, not to restate the code or serve as documentation.

# Tests

Keep tests high-signal and useful during refactors. Exercise the actual implementation and protect observable behavior and contracts that other code depends on. Tests should pass when internals change without changing behavior, and fail when those contracts break. Remove fixture-only consistency checks, tests that recreate implementation logic, and temporary low-signal smoke tests before declaring work complete.
