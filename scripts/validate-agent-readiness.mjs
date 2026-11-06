import { createHash } from "node:crypto";
import { readFile, access } from "node:fs/promises";
import { resolve, dirname } from "node:path";
import { fileURLToPath } from "node:url";

const dist = resolve(process.argv[2] ?? "dist");
const requiredFiles = [
  "robots.txt",
  "sitemap.xml",
  "llms.txt",
  "webmcp.js",
  ".well-known/agent-skills/index.json"
];

for (const file of requiredFiles) {
  await access(resolve(dist, file));
}

const robots = await readFile(resolve(dist, "robots.txt"), "utf8");
for (const token of [
  "Sitemap: https://doanson44.github.io/sitemap.xml",
  "User-agent: OAI-SearchBot",
  "User-agent: GPTBot",
  "User-agent: Claude-SearchBot",
  "User-agent: ClaudeBot",
  "User-agent: PerplexityBot",
  "User-agent: Google-Extended",
  "Content-Signal: ai-train=no, search=yes, ai-input=yes"
]) {
  if (!robots.includes(token)) {
    throw new Error(`robots.txt is missing required directive: ${token}`);
  }
}

const sitemap = await readFile(resolve(dist, "sitemap.xml"), "utf8");
if (
  !sitemap.includes("<urlset") ||
  !sitemap.includes("<loc>https://doanson44.github.io/</loc>")
) {
  throw new Error("sitemap.xml is missing the canonical homepage URL");
}

const indexPath = resolve(
  dist,
  ".well-known/agent-skills/index.json"
);
const index = JSON.parse(await readFile(indexPath, "utf8"));
if (
  index.$schema !==
    "https://schemas.agentskills.io/discovery/0.2.0/schema.json" ||
  !Array.isArray(index.skills)
) {
  throw new Error("Agent Skills discovery index has an unsupported schema");
}

for (const skill of index.skills) {
  if (
    !/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(skill.name) ||
    skill.type !== "skill-md" ||
    typeof skill.url !== "string" ||
    !/^sha256:[a-f0-9]{64}$/.test(skill.digest)
  ) {
    throw new Error(`Invalid Agent Skill index entry: ${skill.name}`);
  }

  const skillPath = resolve(dist, `.${skill.url}`);
  const skillBody = await readFile(skillPath);
  const digest = createHash("sha256").update(skillBody).digest("hex");
  if (`sha256:${digest}` !== skill.digest) {
    throw new Error(`Digest mismatch for Agent Skill: ${skill.name}`);
  }

  const markdown = skillBody.toString("utf8");
  if (
    !markdown.startsWith("---\n") ||
    !markdown.includes(`name: ${skill.name}\n`) ||
    !markdown.includes("description:")
  ) {
    throw new Error(`Invalid SKILL.md frontmatter: ${skill.name}`);
  }
}

const webmcp = await readFile(resolve(dist, "webmcp.js"), "utf8");
if (
  !webmcp.includes("document.modelContext") ||
  !webmcp.includes("get_public_site_capabilities")
) {
  throw new Error("WebMCP capability registration is missing");
}

console.log(
  `Agent readiness assets validated: ${requiredFiles.length} required files, ${index.skills.length} skills.`
);
