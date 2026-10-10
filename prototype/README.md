# Orpheus UI prototype

A standalone prototype for [issue #1](https://github.com/jorikvanveen/oph-trainer/issues/1), built with React, Vite, Tailwind CSS, and shadcn/ui. It lives in `prototype/`, not `frontend/`. This is for exploring the interface and quiz flow; React here does not change the planned SvelteKit production frontend.

## Run

Use Node 22.12 or newer and npm. The repository's Nix development shell includes Node.

```sh
cd prototype
npm install
npm run dev
```

Open the local URL printed by Vite. No environment variables, credentials, backend, or model API key are needed.

## Try the flow

- All topics are selected by default. Choose **Start quiz** to begin, or uncheck topics to narrow the quiz. The reset icon selects all topics again. Deselecting every topic disables quiz start.
- General practice picks up to ten distinct questions. A smaller selected pool produces a shorter quiz. Unanswered and frequently missed questions have higher selection weights; previously correct questions remain eligible.
- Short answers use authored aliases, ignoring capitalization and surrounding whitespace.
- Explanations reveal the reference answer and require self-grading. No automatic explanation grader is connected, and no answer text is sent to a provider.
- Review the reference and official source after answering. A rejected short-answer grade can be marked correct.
- Finish the whole quiz to save its correct/incorrect outcomes. Leaving or refreshing an unfinished quiz discards every outcome in that batch.
- View **Your progress** for question coverage, completed answer counts, and correct-answer percentages overall and by topic.
- Use **Clear progress** on the progress screen to reset completed history, with confirmation.
- Open the official guide from the book icon in the top bar.

The question bank contains 18 syllabus-derived sample questions across all nine knowledge topics, including spectral images and a cue-sheet excerpt. It is not the complete production question bank. Spectral images load directly from the official guide, so viewing those exercises makes a request to `interview.orpheus.network`. Other screens work without external assets or fonts.

## Boundaries

Completed history is stored under `orpheus-ui-prototype.progress.v1` in browser local storage. It contains only quiz identifiers, completion dates, question IDs, and binary outcomes. It does not contain answer text, unfinished quizzes, or grader identity. Clearing browser storage loses guest history. Storage failures are reported, and corrupted history is not silently overwritten.

There is no account interface. The prototype does not create keys, sign in, or synchronize anything. There is no backend, account security implementation, paid grading, deployment setup, or production API contract here. Quiz metrics do not predict admission. The app is for preparation, not use during the real interview.

The components in `src/components/ui/` come from the shadcn registry and are configured by `components.json`. Custom composition is in `src/App.tsx`; the palette and responsive layout are in `src/index.css`.

## Check

```sh
npm test
npm run build
npm run test:browser
```

The browser tests cover topic filtering, short-answer correction, completed-only persistence, self-grading, confirmed reset, guide access, minimal UI copy, and mobile layout. Playwright starts and stops its own Vite server. On NixOS it uses the system Chromium when available. Elsewhere, install Chromium with `npx playwright install chromium`, or set `CHROMIUM_PATH` to your browser executable.

Tests do not contact a grading provider or require secrets.
