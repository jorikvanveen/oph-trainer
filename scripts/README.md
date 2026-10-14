# Project scripts

Run these scripts in the flake devshell. They resolve paths relative to their own location, so they work from any directory.

Install frontend dependencies once with `npm ci --prefix frontend` from the repository root.

Format frontend files with Prettier and its Svelte plugin. Pass paths relative to `frontend/`, or `.` for the whole frontend. Generated DTOs and build outputs are excluded:

```sh
npm run format --prefix frontend -- src/routes/quiz/+page.svelte
npm run format:check --prefix frontend -- src/routes/quiz/+page.svelte
```

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
| `generate-spectrograms.sh <source.flac> [piano.flac]` | Check the source FLACs and regenerate audio-derived spectral exercise PNGs. Requires FFmpeg, SoX and FLAC. |

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

The backend serves the app at `http://127.0.0.1:3000` by default. The development and start scripts leave environment configuration unchanged, including `OPH_BIND_ADDRESS`, `OPH_DATABASE_URL`, `OPH_STATIC_DIR`, `OPH_OPENROUTER_API_KEY`, `OPH_USE_X_FORWARDED_FOR`, and `RUST_LOG`. Relative backend paths resolve from `backend/`. For a trusted reverse proxy, `OPH_USE_X_FORWARDED_FOR=true` selects the first IP in `X-Forwarded-For` for grading rate limits; it defaults to `false`. See [proxy requirements and grading settings](../docs/grading.md).

## Nix package

On x86_64 Linux, build the backend with its static frontend bundled, or run it directly:

```sh
nix build .#oph-trainer
nix run .
```

The default package is `oph-trainer`. The wrapper sets `OPH_STATIC_DIR` to the bundled frontend unless you provide a value. Other environment settings retain their usual defaults. The default SQLite database is created in the process's working directory, which must be writable, or you can set `OPH_DATABASE_URL` to a persistent location.

For a homelab flake, add `inputs.oph-trainer.url = "github:jorikvanveen/oph-trainer"` and use `inputs.oph-trainer.packages.${pkgs.stdenv.hostPlatform.system}.default`. Its executable is `bin/oph-trainer-backend`. Set the service's working directory or database URL to its writable state directory, and configure the bind address for your reverse proxy.

Build only the static frontend with `nix build .#oph-trainer-frontend`. Cargo and npm dependencies come from the committed lockfiles, so dependency updates do not require separate Nix dependency hashes.

### Logging

The backend logs startup stages, question-bank counts, account creation, quiz selection, answer assessment, saved quizzes and progress resets. Each API request has a process-local request ID, method, registered route, response status and elapsed milliseconds. Messages from the same request include its request context. Successful requests log at `info`, HTTP 4xx responses at `warn`, and HTTP 5xx responses at `error`. Static-file requests are excluded to keep page loads quiet. Request timing ends when the response is ready, before its body is sent.

Explanation grading logs provider calls and results. Self-assessment fallback logs a reason, including missing configuration, rate limits, timeouts, provider HTTP failures, oversized responses, invalid decisions and decision confidence below 0.8. Provider HTTP failures also log the status code.

The default filter is `warn,oph_trainer_backend=info`, which shows app activity and dependency warnings. For request starts, progress counts and domain error codes, enable app debug logging:

```sh
RUST_LOG=warn,oph_trainer_backend=debug ./scripts/dev-backend.sh
```

Use `RUST_LOG=warn` for quieter output. Logs omit account keys, authorization headers, submitted answers, provider payloads and bodies, database URLs, arbitrary request paths and query strings. Question IDs in successful assessment logs come from the authored question bank. Avoid enabling dependency debug or trace logging when handling real credentials or answers.

## Browser-to-API tests

Install dependencies with `npm ci --prefix frontend`. Run tests in the flake devshell with a local Chromium browser available:

```sh
./scripts/test.sh
./scripts/test.sh --grep 'automatic account'
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

Dependency and browser installation can require network access. After setup, the default test suite requires no secrets, external assets, OpenRouter/TypeSafe access, or paid model calls. Rust grading tests use a loopback provider stub with a fake key. Browser tests run against the production binary without provider credentials. There is no live-provider smoke test. Chromium runs headless with Playwright's default launch settings; no persistent user browser profile is used.

### Adding feature tests

Put production behavior tests in `frontend/tests/*.spec.ts`. The practice tests load the static page, start quizzes through the same-origin API, and exercise topic selection, immediate feedback, self-assessment, corrected grades, completed-only saving, account access across browser contexts, switching, reset, and keyboard use at mobile sizes. Network failure tests take the browser offline, then retry against the same server.

Use Playwright's `page` fixture, accessible roles and labels, and observable HTTP responses. Drive quizzes, grading, and progress through the UI or public APIs, then verify feedback and persistence by reloading or opening a separate browser context. Do not import private application functions, assert component internals, or query particular SQLite tables. Tests share the run's fresh database and execute with one worker, so create independent account data or reset through public behavior when a feature needs server-side state isolation.

Keep application API endpoints and SQLite real for integration tests. Rust grading tests stub the external provider HTTP endpoint through a Rust constructor, using fixed production limits and timeouts. One focused browser test intercepts explanation answer feedback to check point display and corrections; provider integration and failure cases stay in Rust. No additional browser server or provider process is needed. The production endpoint is fixed and has no environment override.

The React prototype's layout, CSS classes, copy-minimization rules, and screenshots are not production contracts. Keep those assertions on the `frontend-prototype` branch. Feature issues should add their own behavior tests here as their production contracts become available.

## Practice and progress APIs

The practice endpoints are `GET /api/catalog`, `POST /api/quizzes`, and `POST /api/answers`. The catalog returns topic names and stable question/topic IDs. Missing or null `topic_ids` selects general practice across all topics. An empty list or unknown topic ID returns `400` with `invalid_topics`; repeated topic IDs have the same meaning as selecting that topic once. Selection uses recent outcomes from the account identified by its bearer key. Starting practice without a key creates an account and returns its key with the quiz. Client-supplied history is rejected. The backend uses an authored Rust question bank covering all nine official syllabus topics, including local spectral images. General and selected-topic practice select up to ten distinct questions; a smaller pool returns every available question once. An empty pool returns `503` with `no_questions_available`. Quiz drafts stay in browser memory; selection and answer feedback do not save progress.

Quiz size is fixed at ten questions. Smaller pools return every available question once. For each question, use only its last five completed outcomes, ordered oldest to newest. The missed fraction is incorrect outcomes divided by available recent outcomes, or `1` for an unanswered question. Its selection weight is fixed at `1 + 4 * missed_fraction`: `1` for all correct, `3` for half incorrect, and `5` for all incorrect or unanswered. Selection samples by weight without replacement. There are no due dates or review queues.

Submit answers as `{ "question_id": "...", "answer": "..." }`. The server resolves the authored grading criteria by question ID, trims surrounding whitespace and lowercases both short answers and authored variants, and accepts only exact matches against accepted answers or aliases. It does not collapse internal whitespace or use fuzzy matching. Both correct and incorrect feedback contain the authored reference answer and source URL. Explanation grading uses bounded direct Jev decisions when server credentials are configured. One request assesses each required point and contradiction independently. Every decision must have numeric confidence between 0.8 and 1.0 inclusive; any lower, missing or invalid confidence falls back to self-assessment for the whole answer. The backend derives the binary grade and returns every required point with its covered/missed result and authored contradiction text in `explanation_feedback`; these details are shown during the quiz and never saved as progress. Missing configuration, provider failures, and exhausted limits return `needs_self_assessment`, which is ungraded until the learner chooses an outcome. The provider API key and `OPH_USE_X_FORWARDED_FOR` IP-source toggle are configurable; model, endpoint, timeouts, and usage limits are constants. See [grading configuration and limits](../docs/grading.md). Submitted answer text is neither saved nor logged.

Both practice POST endpoints use Axum's standard JSON extractor and rejection responses. They require JSON with `Content-Type: application/json` and reject unknown fields. Malformed JSON returns `400`; missing fields, wrong types, browser-supplied history, or grading criteria return `422`; missing JSON content type returns `415`. Domain errors use `ApiErrorDto`: unknown question IDs return `404` with `unknown_question`; blank answers return `400` with `invalid_answer`.

Edit the authored `QuestionDto` entries in [questions.rs](../backend/src/question_bank/questions.rs) and [syllabus.rs](../backend/src/question_bank/syllabus.rs). Keep existing question IDs for the same questions and assign new IDs to different questions. Review content against the linked official syllabus pages. Ask only about concepts the guide explains; the sample log and cue files do not teach field interpretation, so do not ask learners to read their fields. Author accepted answers and aliases explicitly; explanation criteria require the points the prompt asks for, with optional details and contradictions kept separate. Exercise presentation belongs in `material`, separate from grading. Generated spectrogram PNGs live in `frontend/static/exercises/`. Run `./scripts/check.sh` and `./scripts/test.sh` after edits. Tests check unique IDs, all nine topics, nonblank content, official source links, grading criteria and local asset availability; startup performs no content validation.

The eight pop spectrogram PNGs use the user-supplied `04 - DEAR MISS HOLLOWAY.flac`, its 00:45 to 01:30 excerpt, FFmpeg 9.0.1, LAME 4.0 and SoX 14.4.2. They show the source, four CBR MP3 encodes, V0 and V2 encodes, and MP3 192 CBR converted back to FLAC. Two further PNGs use the supplied `07 - Blues.flac` and the same excerpt times, showing its source spectrum and a known MP3 128 kbps to FLAC transcode. Blues replaces the earlier Köln Concert excerpt after spectral verification showed clearer evidence consistent with a lossless source. The source audio is not bundled. The generated plots show the left channel without mixing, with 3000 time columns, 1025 frequency rows, a Kaiser window and a fixed 0 to -130 dBFS colour scale. Each plot applies the same -6.02 dB analysis gain before integer PCM conversion to prevent decoded MP3 peaks from clipping; the image footer records that gain. This exposes faint high-frequency content previously hidden by the -90 dBFS floor. Encoder defaults differ from the guide's illustrations; the references describe the actual images. The classical exercise bundles the original Orpheus classical diagram as `spectrum-b.jpg`, credited in its description and in the [source analysis](../docs/spectrogram-analysis.md). The piano questions cover sparsity, faint background noise and lossy conversion. The [question audit](../docs/spectrogram-question-audit.md) records the guide basis of every remaining spectral question.

Regenerate all ten PNGs with:

```sh
nix shell nixpkgs#ffmpeg nixpkgs#sox nixpkgs#flac -c ./scripts/generate-spectrograms.sh '.agents/04 - DEAR MISS HOLLOWAY.flac' '.agents/07 - Blues.flac'
```

Omit the second file to regenerate only the eight pop diagrams. The script preserves the source sample rate, encodes MP3 from the unattenuated excerpt, then halves the analysis amplitude before writing 24-bit PCM or FLAC. The FLAC transcode diagrams already have that attenuation and are not attenuated again. It checks FLAC integrity and rejects sample rates other than the 44.1 kHz expected by the authored exercises. It does not download or resize the classical reference. See the [analysis report](../docs/spectrogram-analysis.md) for source verification, measurements and full-track plots.

The [account decision](../docs/adr/0002-account-keys-as-sole-credentials.md) explains account keys. [Backend DTOs](../backend/src/dto.rs) define completed-quiz and progress contracts. When the app opens without a retained key, it creates an account through `POST /api/accounts` before loading practice and progress. Account keys authorize requests through the `Authorization: Bearer` header. The frontend retains only the current account key in local storage. It keeps quiz questions, typed answers, and unsaved outcomes in memory, and reloads completed progress from the server. The Account panel offers the current key for saving and accepts an existing key; a switch takes effect only after the server loads that account's progress. Serve production traffic through HTTPS.

Progress is loaded through `GET /api/progress`. The response contains `completed_quiz_count`, the all-time quiz count, and `recent_outcomes`, each question's last five saved grades in oldest-to-newest order. Quiz selection uses the same per-question query. The progress page displays all-time metrics and topic coverage. SQLite retains all completed history; SeaORM and SeaQuery builders apply account filters, window ranks and aggregates before results are loaded. All progress queries share one read transaction.

`overall` and `by_topic` remain all-time metrics. Overall answer totals and accuracy include retired question IDs, while coverage uses the current question bank. Topic metrics use current topic assignments and exclude retired questions. Empty accounts and empty topics have zero counts and coverage, with null accuracy when there are no applicable outcomes. `POST /api/progress/quizzes` saves a complete batch idempotently; retries with different outcomes return `409`. `DELETE /api/progress` removes all of that account's saved quizzes and outcomes while retaining its key.

The SvelteKit routes are `/` for topic selection, `/quiz/` for the active quiz, `/quiz/summary/` for its completion summary, and `/progress/` for saved metrics. The shared layout holds the account and in-memory batch across route changes. Opening or refreshing a quiz or summary URL without a batch returns to topic selection. Navigation away from an unfinished quiz requires confirmation, including browser Back; confirmed navigation discards the batch.

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
