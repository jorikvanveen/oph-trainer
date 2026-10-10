# Project scripts

Run these scripts in the flake devshell. They resolve paths relative to their own location, so they work from any directory.

Install frontend dependencies once with `npm ci --prefix frontend` from the repository root.

| Script | Task |
| --- | --- |
| `dev-backend.sh` | Run the backend with Cargo. Extra arguments go to `cargo run`. |
| `dev-frontend.sh` | Run the Vite development server. Extra arguments go to Vite. |
| `check.sh` | Check Rust formatting, run workspace Clippy with warnings denied, and check Svelte/TypeScript. |
| `test.sh` | Run Rust workspace tests, build the production app, and run Playwright against Axum with a temporary SQLite database. Extra arguments go to Playwright. |
| `export-dtos.sh` | Regenerate TypeScript DTOs in `frontend/src/lib/dto/` using the ts-rs export tests. |
| `build.sh` | Export DTOs, build the static frontend, and compile the release backend. |
| `start.sh` | Run the release backend. Run `build.sh` first. |
| `migrate.sh` | Run SeaORM migration commands. With no arguments, show migration status. |
| `generate-entities.sh` | Migrate a temporary SQLite database and regenerate SeaORM entities with the CLI. |

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

The backend serves the app at `http://127.0.0.1:3000` by default. The development and start scripts leave `OPH_BIND_ADDRESS`, `OPH_DATABASE_URL`, `OPH_STATIC_DIR`, and `RUST_LOG` unchanged. Relative backend paths resolve from `backend/`.

## Browser-to-API tests

Install dependencies with `npm ci --prefix frontend`. Run tests in the flake devshell with a local Chromium browser available:

```sh
./scripts/test.sh
./scripts/test.sh --grep 'sample API'
```

`npm run test:browser --prefix frontend` invokes the same runner. Do not start Vite or the backend yourself. Direct `playwright test` invocation rejects a missing runner configuration rather than connecting to a developer server.

The runner runs the Rust workspace tests and calls `build.sh`, then starts the release Axum binary serving a copy of the static SvelteKit build. Axum binds `127.0.0.1:0`, so the operating system assigns a free port without a port-selection race. The runner reads the bound address from Axum's startup log and checks that the static frontend responds before starting Playwright. Both the frontend and `/api` use that origin.

Each invocation creates a fresh SQLite file under a unique temporary directory. Axum applies its real migrations before listening. The runner overrides `OPH_BIND_ADDRESS`, `OPH_DATABASE_URL`, `OPH_STATIC_DIR`, and logging settings, and does not pass other operator settings or provider credentials to the service. It never reads `.env` files. Playwright gives every test a new browser context, with empty cookies and browser storage. Browser profiles and the copied static files also live under the temporary directory.

The whole command, including the Rust tests and production build, has a 600-second deadline. Set `OPH_TEST_TIMEOUT_SECONDS` to an integer between 1 and 3600 to change it. Startup has a 30-second limit; the browser suite has a 120-second limit and each test has a 30-second limit. The runner stops its child process groups and removes the temporary workspace on success, failure, deadline expiry, SIGINT, and SIGTERM. Playwright receives SIGINT to close its workers and browsers, with up to ten additional seconds for shutdown before forced termination. Forced termination of the runner with SIGKILL cannot run cleanup. The scripts require a POSIX environment with Bash and Node, as provided by the Linux flake devshell.

Traces and screenshots from failures remain in a unique run directory under `frontend/test-results/`, which Git ignores. The SQLite database and browser profiles do not remain there. Build outputs use the normal `backend/target/` and `frontend/build/` locations. Do not run production builds concurrently with this command, since those build locations are shared.

### Browser setup

The configuration adapts the prototype's Chromium selection without copying its Vite server or presentation tests:

1. Set `CHROMIUM_PATH` to use a particular Chromium executable.
2. Otherwise, use `/run/current-system/sw/bin/chromium` when present, as on the prototype's NixOS setup.
3. Otherwise, use Playwright's managed Chromium. Install it once from `frontend/` with `npx playwright install chromium`. On supported Linux distributions, `npx playwright install --with-deps chromium` also installs system libraries and may require administrator access.

The runner preserves `PLAYWRIGHT_BROWSERS_PATH` and `XDG_CACHE_HOME` for Playwright's installed-browser lookup. Chromium itself uses temporary config and cache directories.

Dependency and browser installation can require network access. After setup, the default smoke test requires no secrets, external assets, OpenRouter/TypeSafe access, or paid model calls. There is no live-provider smoke test. Chromium runs headless with Playwright's default launch settings; no persistent user browser profile is used.

### Adding feature tests

Put production behavior tests in `frontend/tests/*.spec.ts`. The initial smoke test loads the static page, clicks its sample API button, and checks both the browser's same-origin HTTP response and the displayed result. It does not intercept or mock `/api`.

Use Playwright's `page` fixture, accessible roles and labels, and observable HTTP responses. Drive quizzes, grading, and progress through the UI or public APIs, then verify feedback and persistence by reloading or opening a separate browser context. Do not import private application functions, assert component internals, or query particular SQLite tables. Tests share the run's fresh database and execute with one worker, so create independent account data or reset through public behavior when a feature needs server-side state isolation.

Keep application API endpoints and SQLite real. When external grading is implemented, stub only its HTTP provider boundary through the service's configurable provider endpoint. Add that stub's lifecycle and test-only endpoint configuration to this runner, without forwarding live credentials. Do not use browser API interception as a replacement for the backend grading path.

The React prototype's layout, CSS classes, copy-minimization rules, and screenshots are not production contracts. Keep those assertions on the `frontend-prototype` branch. Feature issues should add their own behavior tests here as their production contracts become available.

## Practice and progress APIs

The practice endpoints are `GET /api/catalog`, `POST /api/quizzes`, and `POST /api/answers`. The catalog returns topic names and stable question/topic IDs. Missing or null `topic_ids` selects general practice across all topics. An empty list or unknown topic ID returns `400` with `invalid_topics`; repeated topic IDs have the same meaning as selecting that topic once. Selection uses recent outcomes from the account identified by its bearer key. Starting practice without a key creates an account and returns its key with the quiz. Client-supplied history is rejected. The backend uses a hardcoded Rust question bank: twelve questions across audio formats, MP3, and transcodes. By default general practice selects ten distinct questions; selecting one topic uses its four questions. An empty pool returns `503` with `no_questions_available`. Quiz drafts stay in browser memory; selection and answer feedback do not save progress.

Quiz size is fixed at ten questions. Smaller pools return every available question once. For each question, use only its last five completed outcomes, ordered oldest to newest. The missed fraction is incorrect outcomes divided by available recent outcomes, or `1` for an unanswered question. Its selection weight is fixed at `1 + 4 * missed_fraction`: `1` for all correct, `3` for half incorrect, and `5` for all incorrect or unanswered. Selection samples by weight without replacement. There are no due dates or review queues.

Submit answers as `{ "question_id": "...", "answer": "..." }`. The server resolves the authored grading criteria by question ID, trims surrounding whitespace and lowercases both short answers and authored variants, and accepts only exact matches against accepted answers or aliases. It does not collapse internal whitespace or use fuzzy matching. Both correct and incorrect feedback contain the authored reference answer and source URL. Explanation feedback returns `needs_self_assessment`, which is ungraded until the learner chooses an outcome. No provider calls or credentials are required, and submitted answer text is neither saved nor logged.

Both practice POST endpoints use Axum's standard JSON extractor and rejection responses. They require JSON with `Content-Type: application/json` and reject unknown fields. Malformed JSON returns `400`; missing fields, wrong types, browser-supplied history, or grading criteria return `422`; missing JSON content type returns `415`. Domain errors use `ApiErrorDto`: unknown question IDs return `404` with `unknown_question`; blank answers return `400` with `invalid_answer`.

Edit the authored `QuestionDto` entries in [questions.rs](../backend/src/question_bank/questions.rs). Keep existing question IDs for the same questions, assign new IDs to different questions, and review content against the linked official syllabus pages. Author accepted answers and aliases explicitly; explanation criteria require the points the prompt asks for, with optional details and contradictions kept separate. Run `./scripts/test.sh` after edits. Tests check the bank's unique IDs, topic references, nonblank content, official source links, and grading criteria directly; startup performs no content validation.

The [account decision](../docs/adr/0002-account-keys-as-sole-credentials.md) explains account keys. [Backend DTOs](../backend/src/dto.rs) define completed-quiz and progress contracts. Account keys authorize requests through the `Authorization: Bearer` header. Serve production traffic through HTTPS.

## Migrations

Create migration scaffolds with the CLI, fill their `up` and `down` methods using migration builders, then regenerate entities from the migrated schema:

```sh
./scripts/migrate.sh generate create_accounts
./scripts/generate-entities.sh
```

`generate-entities.sh` creates a fresh SQLite database under a temporary directory, applies all project migrations through `migrate.sh`, and runs `sea-orm-cli generate entity` into `backend/src/entity/`. It ignores `OPH_DATABASE_URL` and removes the temporary database on exit. Generated entities use table names as module names and have no API serialization derives. Keep schema types and relationships in migrations so regeneration reproduces them. Application-specific behavior belongs outside generated entities.

To inspect or apply migrations to the configured application database:

```sh
./scripts/migrate.sh
./scripts/migrate.sh up
```

`migrate.sh` uses `OPH_DATABASE_URL`, or `sqlite://oph-trainer.db?mode=rwc` when unset. This points to the same database as the backend. The backend also applies pending migrations on startup.

`down`, `reset`, `refresh`, and `fresh` can remove tables and data. Check the target database before using them.

The scripts do not read or source `.env` files.
