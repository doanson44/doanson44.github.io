# Rust Formatting Rules

## Purpose

This document is a mandatory project rule for preventing `cargo fmt --check` failures caused by manually editing Rust formatting, especially trailing blank lines and whitespace at the end of files.

## Mandatory Rules

1. **Never manually guess Rust formatting.** `rustfmt` is the source of truth for Rust source formatting.
2. After modifying any `.rs` file, run:

   ```text
   cargo fmt --check
   ```

3. If `cargo fmt --check` reports a diff, do not manually patch individual whitespace changes. Run:

   ```text
   cargo fmt
   ```

   Then run `cargo fmt --check` again.
4. **Do not add or remove blank lines at the end of a Rust file by hand.** A trailing blank line and a single final newline are not interchangeable.
5. A Rust source file must end in the exact form produced by `rustfmt`. Do not infer the required number of final newlines from a CI diff.
6. When a CI log contains a `cargo fmt --check` diff, reproduce the formatting locally with `cargo fmt` rather than trying to interpret the diff and edit whitespace manually.
7. **Never claim formatting validation passed unless `cargo fmt --check` was actually run and returned exit code 0.**
8. Formatting validation must happen after the final code edit. Do not run `cargo fmt --check` before making another Rust source change and then treat the earlier result as valid.

## Required Workflow for Rust Changes

Use this sequence whenever Rust code is changed:

```text
1. Edit the Rust source.
2. Run `cargo fmt`.
3. Run `cargo fmt --check`.
4. If it fails, run `cargo fmt` again and repeat `cargo fmt --check`.
5. Only continue to compilation/tests after `cargo fmt --check` passes.
```

## CI Failure Handling

If CI reports something such as:

```text
Diff in .../src/app.rs:...
...
Error: Process completed with exit code 1.
```

and the diff consists only of whitespace or blank-line changes:

- Do not manually add or remove a newline based on visual interpretation of the diff.
- Run `cargo fmt` on the repository.
- Run `cargo fmt --check` again.
- Inspect the resulting diff only if the check still fails.
- Do not make another formatting change without rerunning the check.

## Important Incident: `src/app.rs`

A previous fix incorrectly added an extra blank line after the final closing brace in `src/app.rs`. CI then reported that `cargo fmt --check` wanted that blank line removed. The follow-up fix removed the wrong newline manually, which caused another formatting mismatch.

The lesson is definitive: **do not manually manipulate final newlines to satisfy a `rustfmt` diff. Always let `cargo fmt` make the formatting change, then verify with `cargo fmt --check`.**

## Validation Standard

For this project, formatting is not considered complete until the exact command below has been executed successfully:

```text
cargo fmt --check
```

A CI failure means the formatting state must be treated as invalid until the command is rerun successfully after the final edit.
