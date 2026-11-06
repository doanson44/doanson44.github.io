import test from "node:test";
import assert from "node:assert/strict";
import worker from "./src/index.js";

test("health endpoint returns service metadata", async () => {
  const response = await worker.fetch(new Request("https://api.example/health"));
  assert.equal(response.status, 200);
  const payload = await response.json();
  assert.equal(payload.status, "ok");
  assert.equal(payload.service, "doanson44-market-data-api");
});

test("unknown route returns 404", async () => {
  const response = await worker.fetch(new Request("https://api.example/unknown"));
  assert.equal(response.status, 404);
});

test("non-GET methods are rejected", async () => {
  const response = await worker.fetch(new Request("https://api.example/health", { method: "POST" }));
  assert.equal(response.status, 405);
});

test("untrusted browser origins are rejected", async () => {
  const response = await worker.fetch(new Request("https://api.example/health", {
    headers: { Origin: "https://attacker.example" },
  }));
  assert.equal(response.status, 403);
});

test("OpenAPI document is served as JSON", async () => {
  const response = await worker.fetch(new Request("https://api.example/openapi.json"));
  assert.equal(response.status, 200);
  const payload = await response.json();
  assert.equal(payload.openapi, "3.1.0");
  assert.ok(payload.paths["/v1/crypto/futures/analysis"]);
});
