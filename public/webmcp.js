(() => {
  const modelContext = document.modelContext;

  if (!modelContext || typeof modelContext.registerTool !== "function") {
    return;
  }

  modelContext.registerTool({
    name: "get_public_site_capabilities",
    title: "Get public site capabilities",
    description:
      "Return a read-only summary of public sections and operational boundaries of doanson44.github.io.",
    inputSchema: {
      type: "object",
      properties: {},
      additionalProperties: false
    },
    annotations: {
      readOnlyHint: true,
      untrustedContentHint: false
    },
    execute: async () => ({
      site: "https://doanson44.github.io/",
      application: "Client-side Rust and Leptos CSR/WASM developer utility hub",
      routes: [
        "/",
        "/#/tools",
        "/#/tools/markdown",
        "/#/tools/json",
        "/#/tools/jwt",
        "/#/tools/base64",
        "/#/games",
        "/#/cv",
        "/#/socket"
      ],
      capabilities: [
        "Browser-based Markdown, JSON, JWT, and Base64 utilities",
        "Browser games",
        "Public read-only market-data playground"
      ],
      boundaries: [
        "No first-party server-side REST API is published",
        "The market-data playground does not place trades",
        "No OAuth/OIDC authorization server or MCP server is published",
        "JWT decoding does not by itself verify token signatures",
        "Base64 is encoding, not encryption"
      ],
      discovery: {
        robots: "https://doanson44.github.io/robots.txt",
        sitemap: "https://doanson44.github.io/sitemap.xml",
        agentSkills:
          "https://doanson44.github.io/.well-known/agent-skills/index.json",
        aiCatalog: "https://doanson44.github.io/.well-known/ai-catalog.json",
        llms: "https://doanson44.github.io/llms.txt"
      }
    })
  }).catch((error) => {
    console.warn("WebMCP capability registration failed:", error);
  });
})();
