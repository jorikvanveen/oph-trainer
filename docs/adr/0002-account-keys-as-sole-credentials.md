# Account keys as direct credentials

An account key identifies an account and authorizes access to its progress. Send it with each account API request as a bearer credential. Accounts store only practice progress, so conventional usernames, email addresses, separate passwords, session records, and external identity providers add more account-management work than this app needs. Account keys are secrets, not public identifiers.

Every learner has an account. When the web app opens without a retained account key, it creates an account automatically before loading practice and progress, with no registration step. It retains the generated key so the client can send it with later requests. SQLite stores completed history, and the backend derives metrics and recent outcomes from it. Clients keep the key while learners use an account; browser code can read it. HTTPS protects keys in transit.

Sending another account key switches to that account without merging histories. Losing the key means losing access to that account; there is no recovery service. A database copy contains working account keys. This trade-off is accepted because accounts contain only practice progress and the simpler design avoids a separate session lifecycle.
