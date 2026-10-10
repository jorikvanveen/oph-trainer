# Account keys as sole credentials

An account key both identifies an account and authorizes access to its progress. Accounts store only practice progress, so conventional usernames, email addresses, separate passwords, and external identity providers add more account-management work than this app needs. Account keys are secrets, not public identifiers.

Every learner has an account. The app creates one automatically when practice starts without a signed-in session, with no registration step. Saving the generated key is optional and does not block practice. SQLite is the sole store of completed history, and the backend derives metrics and recent outcomes from it. The browser retains a secure HttpOnly session cookie and transient page state, not progress history or a durable metrics cache. This requires server access to save progress, but avoids separate guest and account persistence paths and synchronization between them.

Entering an existing account key switches the session to that account and loads its saved progress without merging histories. The previous account's saved progress remains unchanged. Users can replace a key while authenticated without losing progress. Clearing browser data removes the session, not the server history. Losing the key and all authenticated sessions means losing access; there is no recovery service.
