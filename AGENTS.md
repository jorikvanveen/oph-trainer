# Project scripts

Prefer the scripts in `scripts/` for common development tasks.

Before running the app, checking or building changes, exporting DTOs, or managing migrations, read `scripts/README.md` for the available commands and usage.

# Comments

Use comments only to explain non-obvious details or reasoning, not to restate the code or serve as documentation.

# Tests

Keep tests high-signal and useful during refactors. Exercise the actual implementation and protect observable behavior and contracts that other code depends on. Tests should pass when internals change without changing behavior, and fail when those contracts break. Remove fixture-only consistency checks, tests that recreate implementation logic, and temporary low-signal smoke tests before declaring work complete.
