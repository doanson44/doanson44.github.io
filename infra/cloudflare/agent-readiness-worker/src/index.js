const ORIGIN = "https://doanson44.github.io";

const DISCOVERY_LINKS = [
  '<https://doanson44.github.io/.well-known/agent-skills/index.json>; rel="agent-skills"; type="application/json"',
  '<https://doanson44.github.io/sitemap.xml>; rel="sitemap"; type="application/xml"'
].join(", ");

export default {
  async fetch(request) {
    const incomingUrl = new URL(request.url);
    const originUrl = new URL(incomingUrl.pathname + incomingUrl.search, ORIGIN);

    const upstream = await fetch(
      new Request(originUrl, {
        method: request.method,
        headers: request.headers,
        redirect: "follow"
      })
    );

    const headers = new Headers(upstream.headers);
    headers.set("Content-Signal", "ai-train=no, search=yes, ai-input=yes");
    headers.set("Vary", appendVary(headers.get("Vary"), "Accept"));

    if (incomingUrl.pathname === "/") {
      headers.append("Link", DISCOVERY_LINKS);
    }

    if (
      incomingUrl.pathname.startsWith("/.well-known/agent-skills/") ||
      incomingUrl.pathname === "/llms.txt"
    ) {
      headers.set("Access-Control-Allow-Origin", "*");
      headers.set("Access-Control-Allow-Methods", "GET, HEAD, OPTIONS");
      headers.set("Access-Control-Allow-Headers", "Accept, Content-Type");
    }

    if (request.method === "OPTIONS") {
      return new Response(null, {
        status: 204,
        headers: {
          "Access-Control-Allow-Origin": "*",
          "Access-Control-Allow-Methods": "GET, HEAD, OPTIONS",
          "Access-Control-Allow-Headers": "Accept, Content-Type",
          "Access-Control-Max-Age": "86400"
        }
      });
    }

    return new Response(upstream.body, {
      status: upstream.status,
      statusText: upstream.statusText,
      headers
    });
  }
};

function appendVary(current, value) {
  const values = (current || "")
    .split(",")
    .map((item) => item.trim())
    .filter(Boolean);

  if (!values.some((item) => item.toLowerCase() === value.toLowerCase())) {
    values.push(value);
  }

  return values.join(", ");
}
