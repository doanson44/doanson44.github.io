---
name: smart-build
description: "Automatically trigger a full build cycle: git pull, then trunk build --release, and auto-fix any compilation errors. Use when: the user types 'build', 'smart build', or requests an automatic pull-and-build cycle with error fixing."
---

# Smart Build Skill

When the user triggers this skill (e.g., by typing `build`), you MUST follow these steps exactly:

1. **Pull Latest Changes**:
   - Run `git pull` in the repository to ensure the local branch is up-to-date.

2. **Quality Check**:
   - Run `cargo clippy --target wasm32-unknown-unknown -- -D warnings` to catch linting errors before building.

3. **Run Build**:
   - Run the production build command: `trunk build --release`.

4. **Check for Errors and Auto-Fix**:
   - Wait for the commands to complete.
   - If both the `cargo clippy` and `trunk build` succeed, notify the user.
   - If either fails with compilation or linting errors (e.g., Rust compiler errors, missing dependencies, clippy warnings):
     - Analyze the error output carefully.
     - Use your editing tools to fix the code automatically.
     - After applying fixes, loop back to Step 2 to re-run the checks and build.
     - Continue this process until both commands succeed.

5. **Completion**:
   - Once the build completes successfully, report back to the user that the pull, check, build, and any necessary fixes were completed successfully.
