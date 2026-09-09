const { spawnSync } = require("node:child_process");

const result = spawnSync(
  "npm",
  ["run", "build:css"],
  {
    stdio: "inherit",
    shell: process.platform === "win32",
  },
);

if (result.error) {
  console.error(result.error);
  process.exit(1);
}

if (result.status !== 0) {
  process.exit(result.status ?? 1);
}
