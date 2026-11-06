import { createHash } from "node:crypto";
import { readFile, access } from "node:fs/promises";
import { resolve } from "node:path";

const dist = resolve(process.argv[2] ?? "dist");
const requiredFiles = [
  "robots.txt",
  "sitemap.xml",
  "llms.txt",
  "webmcp.js",
  ".well-known/agent-skills/index.json",
  ".well-known/ai-catalog.json"
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

const skillNames = new Set();

for (const skill of index.skills) {
  if (
    !/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(skill.name) ||
    skillNames.has(skill.name) ||
    skill.type !== "skill-md" ||
    typeof skill.url !== "string" ||
    !/^sha256:[a-f0-9]{64}$/.test(skill.digest)
  ) {
    throw new Error(`Invalid Agent Skill index entry: ${skill.name}`);
  }

  skillNames.add(skill.name);
  if (
    !skill.url.startsWith("/.well-known/agent-skills/") ||
    skill.url.includes("..")
  ) {
    throw new Error(`Skill URL is outside the Agent Skills namespace: ${skill.name}`);
  }

  const skillPath = resolve(dist, skill.url.replace(/^\/+/, ""));
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

const catalogPath = resolve(dist, ".well-known/ai-catalog.json");
const catalog = JSON.parse(await readFile(catalogPath, "utf8"));
if (
  catalog.specVersion !== "1.0" ||
  typeof catalog.host?.displayName !== "string" ||
  !Array.isArray(catalog.entries) ||
  catalog.entries.length !== index.skills.length
) {
  throw new Error("ARD AI Catalog has an invalid top-level structure");
}

const indexedSkillUrls = new Set(index.skills.map((skill) => skill.url));
const catalogIdentifiers = new Set();
const catalogSkillUrls = new Set();

for (const entry of catalog.entries) {
  const hasUrl = typeof entry.url === "string";
  const hasData = Object.hasOwn(entry, "data");
  if (
    !/^urn:air:doanson44\.github\.io:skill:[a-z0-9-]+$/.test(entry.identifier) ||
    catalogIdentifiers.has(entry.identifier) ||
    typeof entry.displayName !== "string" ||
    entry.type !== "text/markdown" ||
    hasUrl === hasData ||
    !Array.isArray(entry.representativeQueries) ||
    entry.representativeQueries.length < 2 ||
    entry.representativeQueries.length > 5 ||
    entry.representativeQueries.some((query) => typeof query !== "string" || !query.trim())
  ) {
    throw new Error(`Invalid ARD catalog entry: ${entry.identifier}`);
  }

  catalogIdentifiers.add(entry.identifier);
  if (hasUrl) {
    const prefix = "https://doanson44.github.io";
    if (!entry.url.startsWith(prefix)) {
      throw new Error(`ARD entry URL is outside the canonical site: ${entry.identifier}`);
    }
    const skillUrl = entry.url.slice(prefix.length);
    if (!indexedSkillUrls.has(skillUrl)) {
      throw new Error(`ARD entry does not resolve to an indexed Agent Skill: ${entry.identifier}`);
    }
    catalogSkillUrls.add(skillUrl);
  }
}

if (catalogSkillUrls.size !== index.skills.length) {
  throw new Error("ARD catalog must publish every indexed Agent Skill exactly once");
}

const html = await readFile(resolve(process.cwd(), "index.html"), "utf8");
if (!html.includes('rel="ai-catalog"') || !html.includes('/.well-known/ai-catalog.json')) {
  throw new Error("Homepage is missing the HTML AI Catalog discovery link");
}

const edgeWorker = await readFile(
  resolve(process.cwd(), "infra/cloudflare/agent-readiness-worker/src/index.js"),
  "utf8"
);
for (const token of [
  'rel="ai-catalog"',
  'rel="sitemap"',
  'Content-Signal',
  'application/ai-catalog+json',
  'Access-Control-Allow-Origin'
]) {
  if (!edgeWorker.includes(token)) {
    throw new Error(`Cloudflare Worker template is missing discovery behavior: ${token}`);
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
