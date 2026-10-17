# Orpheus interview trainer

> [!IMPORTANT]
> The question bank is now frozen. I created the entire existing question bank before ever participating in an Orpheus interview. I have since taken and passed the interview, so I will no longer maintain the question bank to avoid any perception that I am sharing interview questions.
>
> I also cannot accept question bank contributions, as I cannot verify whether a contributor has ever participated in an Orpheus interview. You are free to clone this repository and host your own question bank.
>
> I will continue maintaining the app and making functional improvements.

[oph-trainer.jorik-dev.com](https://oph-trainer.jorik-dev.com/)

An unofficial practice app for the [Orpheus admission interview](https://interview.orpheus.network/). Study the official syllabus with short quizzes, answer feedback, and saved progress. Not affiliated with Orpheus.

- Practice all nine syllabus topics, or select the topics you want to study.
- Answer up to ten questions per quiz, with selection weighted toward questions you have missed or have not practiced.
- Review reference answers and links to the official guide. Correct grades or assess explanations yourself when needed.
- Track topic coverage and accuracy from completed quizzes. Use your account key to access the same progress on another device.

Quizzes are daily practice, rather than full mock interviews. Your account key is created automatically. Save it somewhere private; there is no account recovery if you lose it.

## Run locally

The Nix flake provides Rust, Node.js, and the development tools. From the repository root:

```sh
nix develop
npm ci --prefix frontend
```

Start the backend and frontend in separate terminals using that environment:

```sh
./scripts/dev-backend.sh
```

```sh
./scripts/dev-frontend.sh
```

Open the URL printed by Vite. The frontend forwards API requests to the backend at `http://127.0.0.1:3000`.

SQLite is created and migrated automatically on startup. Short answers are graded locally. To enable automatic explanation grading, set `OPH_OPENROUTER_API_KEY` in the backend's environment. Without a key, explanations use self-assessment.

## Build and run

Inside the development environment, with frontend dependencies installed:

```sh
./scripts/build.sh
./scripts/start.sh
```

Open `http://127.0.0.1:3000`. The Rust service serves both the API and the static frontend.

On x86_64 Linux, you can also build and run the bundled Nix package:

```sh
nix build .#oph-trainer
nix run .
```

For deployment, put the service behind an HTTPS reverse proxy and keep its SQLite database in a persistent, writable location. See [the scripts guide](scripts/README.md) for environment settings and homelab setup.

## Development

The frontend uses SvelteKit and TypeScript, with HTML prerendered at build time. The backend uses Rust, Axum, SeaORM, and SQLite. Quiz state stays in browser memory; completed outcomes are saved on the server.

```sh
./scripts/check.sh
./scripts/test.sh
```

The test runner builds the production app, runs Rust tests, and tests the UI against Axum with Playwright and a temporary database. Browser tests need Chromium; see [browser setup](scripts/README.md#browser-setup).

- [Scripts and development workflows](scripts/README.md)
- [Project terminology](CONTEXT.md)
- [Architecture decisions](docs/adr/)
- [Question bank](backend/src/question_bank/)
