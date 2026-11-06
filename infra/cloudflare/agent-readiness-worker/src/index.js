const ORIGIN = "https://doanson44.github.io";

export default {
  async fetch(request) {
    const incomingUrl = new URL(request.url);
    const isDiscoveryResource =
      incomingUrl.pathname.startsWith("/.well-known/agent-skills/") ||
      incomingUrl.pathname === "/llms.txt";

    if (request.method === "OPTIONS") {
      if (!isDiscoveryResource) {
        return new Response(null, { status: 204 });
      }

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

    if (request.method !== "GET" && request.method !== "HEAD") {
      return new Response("Method Not Allowed", {
        status: 405,
        headers: { Allow: "GET, HEAD, OPTIONS" }
      });
    }

    const originUrl = new URL(
      incomingUrl.pathname + incomingUrl.search,
      ORIGIN
    );
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
      const discoveryLinks = [
        `<${incomingUrl.origin}/.well-known/agent-skills/index.json>; rel="agent-skills"; type="application/json"`,
        `<${incomingUrl.origin}/sitemap.xml>; rel="sitemap"; type="application/xml"`
      ];
      for (const link of discoveryLinks) {
        headers.append("Link", link);
      }
    }

    if (isDiscoveryResource) {
      headers.set("Access-Control-Allow-Origin", "*");
      headers.set("Access-Control-Allow-Methods", "GET, HEAD, OPTIONS");
      headers.set("Access-Control-Allow-Headers", "Accept, Content-Type");
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
