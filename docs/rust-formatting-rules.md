# Rust Coding and Formatting Rules

## Purpose

This is a coding rule, not a substitute for CI or a requirement for an unavailable local shell environment.

The goal is to write Rust source in a way that naturally remains compatible with `rustfmt` and avoids introducing formatting-only CI failures.

## Mandatory Rules

1. **Treat `rustfmt` as the source of truth.** Never invent a formatting convention when Rust tooling already defines it.
2. **Do not manually manipulate whitespace to satisfy a `rustfmt` diff.** If formatting needs to change, use `cargo fmt` when a shell environment is available.
3. **Never manually add or remove blank lines at the end of a Rust file.** In particular, do not guess whether a file needs one final newline or an additional blank line.
4. **Do not use trailing whitespace.** Keep indentation and line breaks consistent with normal `rustfmt` output.
5. **Keep Rust source structurally rustfmt-friendly while editing.** Prefer normal Rust formatting instead of compressed one-line expressions when the formatter would expand them.
6. **Do not reformat unrelated code.** When implementing a feature or fix, keep the change focused. If `cargo fmt` changes unrelated formatting, review that diff before including it.
7. **Do not claim formatting validation merely from visual inspection.** Code inspection can establish that the code is plausibly formatted, but only `cargo fmt --check` or equivalent CI output proves formatting validation.
8. **After the final Rust edit, formatting must be considered unverified until `cargo fmt --check` or CI has actually checked it.** Never use an earlier formatting result to claim that a later edit is formatted.

## File Ending Rule

Every `.rs` file must have exactly the file ending produced by `rustfmt`.

Do not reason about this manually. Specifically:

- Do not add an extra empty line after the final `}` just because a file appears visually cleaner that way.
- Do not remove the final newline because a CI diff appears to show a blank line.
- Do not attempt to fix a `cargo fmt --check` failure by guessing which newline is wrong.
- If a formatting tool is available, let the tool make the change.

## Editing Workflow

When a shell environment is available:

```text
Edit → cargo fmt → cargo fmt --check
```

When no shell environment is available:

```text
Edit carefully according to these rules → do not claim formatting validation → let CI verify it
```

The second workflow is intentional. Do not pretend that a command was executed when the environment does not provide command execution.

## CI Failure Handling

If CI reports a `cargo fmt --check` failure:

1. Treat the current formatting as invalid.
2. Do not manually interpret the final-newline diff and patch it by guesswork.
3. If shell execution is available, run `cargo fmt`.
4. Re-run `cargo fmt --check` after the final edit.
5. If shell execution is unavailable, make the smallest source-level correction based on the formatter output available through CI and have CI verify it again.
6. Do not report the formatting check as passed until a real command result or CI result confirms it.

## Other Validation Commands

The following commands are **validation steps**, not coding rules:

```text
cargo fmt --check
cargo check --target wasm32-unknown-unknown
cargo test
cargo clippy --target wasm32-unknown-unknown -- -D warnings
trunk build --release
```

They must not be presented as having been executed unless the execution environment or CI provides actual results.

## Incident Prevention

A previous change to `src/app.rs` caused repeated CI failures because the final newline was manually adjusted based on the visual appearance of a `cargo fmt --check` diff.

The permanent rule is:

> **Never fix Rust formatting by guessing at whitespace. Write normal Rust, use `cargo fmt` when available, and rely on actual formatter/CI output for validation.**
