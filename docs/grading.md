# Explanation grading

The server sends explanation answers and authored grading context to OpenRouter and TypeSafe. One direct Jev Decisions request contains a separate Choice decision for each required point and each authored contradiction. Required points return `covered` or `missing`; contradictions return `asserted` or `not_asserted`. The backend computes a correct grade only when every required point is covered and no contradiction is asserted. Optional supporting details do not affect the grade.

A required point may be stated explicitly or clearly implied by the explanation in the context of the question. The grader must not demand particular keywords or a separate restatement of a direct consequence already established, and must not invent reasoning to fill a gap. Wording and grammar may differ. Negating or rejecting a false claim does not count as asserting a contradiction.

Automatic explanation feedback includes `explanation_feedback`, with `required_points` containing every assessed point as `{ point, covered }`, and `contradictions` containing any detected false claims as authored text. The quiz screen shows every required point as Covered or Missed for both correct and incorrect explanations, alongside any detected contradictions and the reference answer. Marking the answer correct retains the original automatic assessment and identifies the learner's correction. Short answers and ungraded fallback responses omit this field. For ungraded explanations, the screen still lists the authored required points as Not graded, including after the learner chooses their own final outcome. The details stay in browser memory and are never included in completed progress; only final binary outcomes are saved. The app does not expose or retain model probabilities, confidence, provider identity, or answer text, and does not display model-written explanations.

The [official Decisions reference](https://openrouter.ai/docs/api/api-reference/alphadecisions/submit-a-decisions-request) and [TypeSafe state documentation](https://docs.typesafe.ai/concepts/state) support multiple independent questions sharing one state. The request contains `model`, `state`, and `questions`; decisions come from the matching keys in `answers`. Every expected decision must have type `choice` and a recognized value. Missing, extra, or invalid decisions discard the entire assessment and use self-assessment without partial diagnostic feedback. Provider fallbacks are disabled. There are no chat calls or application retries.

Short answers use authored deterministic matching. Unknown questions, blank answers, oversized answers, and invalid JSON make no provider calls. The backend resolves all grading context from its question bank, regardless of what the browser displays.

## Configuration

Set `OPH_OPENROUTER_API_KEY` for automatic grading. A missing or blank key disables it. Credentials stay on the server, and scripts do not read `.env` files.

Set `OPH_USE_X_FORWARDED_FOR=true` behind a trusted reverse proxy to use the first comma-separated IP in `X-Forwarded-For` for rate limiting. It accepts IPv4 or IPv6 and trims surrounding whitespace. Missing, empty, or malformed first entries fall back to the socket IP. The default is `false`, which ignores forwarding headers. The setting accepts only `true` or `false`; other values fail startup.

Enable this only when the backend is accessible solely through a trusted proxy that overwrites `X-Forwarded-For` with the visitor's IP. The app trusts this header whenever the toggle is enabled; callers with direct backend access could otherwise choose their own rate-limit keys. `Forwarded` and `X-Real-IP` are always ignored.

The remaining grading settings are constants in [grading.rs](../backend/src/grading.rs):

| Setting | Fixed value |
| --- | --- |
| Decisions endpoint | `https://openrouter.ai/api/alpha/decisions` |
| Direct Jev model | `typesafe/jev-1.13` |
| Request and response deadline | 8 seconds |
| Provider response limit | 65,536 bytes |
| Answer request body limit | 16,384 bytes, with Axum's standard `413` rejection |
| Answer length limit | 4,000 Unicode scalar values, including whitespace; longer answers return `400` with `invalid_answer` |
| Grading attempt limit | 60 per IP address per 60 seconds |

Redirects, retries, and environment HTTP proxies are disabled. Model and endpoint selection are fixed in production. Tests replace only the external HTTP endpoint through a Rust constructor; production configuration exposes no endpoint override.

## Limits and reset behavior

The server allows 60 grading attempts per IP address per minute. Each explanation consumes one attempt before the provider request, regardless of how many criteria are assessed. Failed requests, invalid responses, and timeouts still count. Deterministic grading, invalid submissions, missing credentials, and self-assessment consume no attempts. There is no application-wide budget or concurrency cap; the operator manages spending through the provider API key's dollar cap.

Counters are in memory and apply to one server process. Restarting resets them; multiple instances have separate quotas. Each IP's fixed window begins with its first grading attempt and expires after 60 seconds. Expired records are removed when checking the next attempt.

Client identity uses the socket IP by default, or the forwarded IP when `OPH_USE_X_FORWARDED_FOR=true`, independent of ports and account keys. With the default setting, clients behind a reverse proxy share its rate bucket. If no usable forwarded IP or connection metadata is available, requests share one rate bucket.

Missing credentials, transport errors, non-success responses, invalid decisions, timeouts, and exhausted limits return the authored reference answer with `needs_self_assessment`. The learner must choose correct or incorrect before advancing. An oversized submission remains ungraded; the quiz screen offers self-assessment using its authored reference, so rejection cannot block completion. Learners can also choose self-assessment before sending an explanation. Final binary outcomes retain the existing completed-only, atomic, idempotent save contract.

## Tests

`./scripts/test.sh` runs Rust tests, builds the production app, and runs browser tests against Axum with an isolated SQLite database and no provider credentials. Rust grading tests replace the external HTTP endpoint with a loopback stub to check decision mapping, request context, invalid responses, input limits, the 60-request IP quota, both IP-source modes, forwarded-header parsing and socket fallback, and timeout fallback. One unit test covers independent IP quotas and their reset after a minute without waiting in real time.

One focused browser test supplies explanation feedback at `/api/answers` to check Covered/Missed points, contradictions, corrections, and outcome-only saving. Account creation, quiz selection, short-answer grading, and progress persistence still use the real API. The existing practice test checks that ungraded points remain Not graded after self-assessment. Provider failure cases are tested once in Rust rather than repeated through full browser quizzes. Default tests never call a paid provider; they verify application behavior rather than Jev's grading accuracy.
