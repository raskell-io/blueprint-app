@AGENTS.md

## For Claude Code

- Rules by task: `.claude/rules/rust-wasm.md` for anything under `crates/`, `.claude/rules/svelte.md` for `src/`.
- Verify in a real browser before reporting done (Playwright or Chrome headless): renders, console, offline reload.
  Note that the CSP blocks `eval`, so Playwright's `waitForFunction` fails here; poll with `page.$eval` instead.
