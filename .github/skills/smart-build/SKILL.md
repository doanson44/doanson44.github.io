---
name: smart-build
description: "Automatically trigger a full build cycle: git pull, then trunk build --release, and auto-fix any compilation errors. Use when: the user types 'build', 'smart build', or requests an automatic pull-and-build cycle with error fixing."
---

# Smart Build Skill

When the user triggers this skill (e.g., by typing `build`), you MUST follow these steps exactly:

1. **Pull Latest Changes**:
   - Run `git pull` in the repository to ensure the local branch is up-to-date.

2. **Run Build**:
   - Run the production build command: `trunk build --release`.

3. **Check for Errors and Auto-Fix**:
   - Wait for the build command to complete.
   - If the build succeeds, notify the user.
   - If the build fails with compilation errors (e.g., Rust compiler errors, missing dependencies):
     - Analyze the error output carefully.
     - Use your editing tools to fix the code automatically.
     - After applying fixes, loop back to Step 2 to re-run the build.
     - Continue this process until the build succeeds.

4. **Completion**:
   - Once the build completes successfully, report back to the user that the pull, build, and any necessary fixes were completed successfully.
