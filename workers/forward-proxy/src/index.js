const ALLOWED_ORIGINS = new Set([
  "https://doanson44.github.io",
]);

const ALLOWED_HOSTS = new Set([
  "api.mexc.com",
]);

export default {
  async fetch(request) {
    const origin = request.headers.get("Origin");

    if (request.method === "OPTIONS") {
      return handleOptions(origin);
    }

    if (request.method !== "POST") {
      return json(
        {
          error: "Method Not Allowed",
        },
        405,
        origin,
      );
    }

    if (!origin || !ALLOWED_ORIGINS.has(origin)) {
      return json(
        {
          error: "Origin Not Allowed",
        },
        403,
        origin,
      );
    }

    let payload;

    try {
      payload = await request.json();
    } catch {
      return json(
        {
          error: "Invalid JSON body",
        },
        400,
        origin,
      );
    }

    const targetUrl = payload?.targetUrl;
    const method = String(payload?.method || "GET").toUpperCase();

    if (typeof targetUrl !== "string") {
      return json(
        {
          error: "targetUrl is required",
        },
        400,
        origin,
      );
    }

    let target;

    try {
      target = new URL(targetUrl);
    } catch {
      return json(
        {
          error: "Invalid targetUrl",
        },
        400,
        origin,
      );
    }

    if (target.protocol !== "https:") {
      return json(
        {
          error: "Only HTTPS targets are allowed",
        },
        400,
        origin,
      );
    }

    if (!ALLOWED_HOSTS.has(target.hostname)) {
      return json(
        {
          error: "Target host is not allowed",
        },
        403,
        origin,
      );
    }

    if (!["GET", "POST", "PUT", "PATCH", "DELETE"].includes(method)) {
      return json(
        {
          error: "HTTP method is not allowed",
        },
        400,
        origin,
      );
    }

    const headers = new Headers();

    headers.set("Accept", "application/json");

    if (payload?.headers && typeof payload.headers === "object") {
      for (const [key, value] of Object.entries(payload.headers)) {
        if (typeof value === "string") {
          headers.set(key, value);
        }
      }
    }

    const init = {
      method,
      headers,
    };

    if (method !== "GET" && method !== "HEAD" && payload?.body != null) {
      init.body =
        typeof payload.body === "string"
          ? payload.body
          : JSON.stringify(payload.body);
    }

    let upstreamResponse;

    try {
      upstreamResponse = await fetch(target.toString(), init);
    } catch (error) {
      return json(
        {
          error: "Upstream request failed",
          message: String(error),
        },
        502,
        origin,
      );
    }

    const responseHeaders = new Headers(upstreamResponse.headers);

    responseHeaders.set(
      "Access-Control-Allow-Origin",
      origin,
    );

    responseHeaders.set(
      "Vary",
      "Origin",
    );

    responseHeaders.set(
      "Cache-Control",
      "no-store",
    );

    return new Response(
      upstreamResponse.body,
      {
        status: upstreamResponse.status,
        statusText: upstreamResponse.statusText,
        headers: responseHeaders,
      },
    );
  },
};

function handleOptions(origin) {
  if (!origin || !ALLOWED_ORIGINS.has(origin)) {
    return new Response(null, {
      status: 403,
    });
  }

  return new Response(null, {
    status: 204,
    headers: {
      "Access-Control-Allow-Origin": origin,
      "Access-Control-Allow-Methods": "POST, OPTIONS",
      "Access-Control-Allow-Headers": "Content-Type, Accept",
      "Access-Control-Max-Age": "86400",
      "Vary": "Origin",
    },
  });
}

function json(body, status, origin) {
  const headers = {
    "Content-Type": "application/json",
  };

  if (origin && ALLOWED_ORIGINS.has(origin)) {
    headers["Access-Control-Allow-Origin"] = origin;
    headers["Vary"] = "Origin";
  }

  return new Response(
    JSON.stringify(body),
    {
      status,
      headers,
    },
  );
}
