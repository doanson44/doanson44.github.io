import test, { mock } from "node:test";
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

test("OpenAPI document is served with the request origin", async () => {
  const response = await worker.fetch(new Request("https://api.example/openapi.json"));
  assert.equal(response.status, 200);
  const payload = await response.json();
  assert.equal(payload.openapi, "3.1.0");
  assert.equal(payload.servers[0].url, "https://api.example");
  assert.ok(payload.components.schemas.Candle);
  assert.ok(payload.components.schemas.AnalysisResponse);
  assert.ok(payload.paths["/v1/crypto/futures/analysis"]);
});

test("analysis endpoint returns normalized candles and closed-candle indicators", async () => {
  const now = Math.floor(Date.now() / 1000);
  const currentHour = Math.floor(now / 3600) * 3600;
  const firstOpenTime = currentHour - 40 * 3600;
  const times = [];
  const open = [];
  const high = [];
  const low = [];
  const close = [];
  const vol = [];

  for (let index = 0; index < 40; index += 1) {
    const price = 100 + index;
    times.push(firstOpenTime + index * 3600);
    open.push(price - 0.25);
    high.push(price + 1);
    low.push(price - 1);
    close.push(price);
    vol.push(1000 + index);
  }

  const upstreamPayload = {
    success: true,
    code: 0,
    data: { time: times, open, high, low, close, vol },
  };

  mock.method(globalThis, "fetch", async () => new Response(JSON.stringify(upstreamPayload), {
    status: 200,
    headers: { "Content-Type": "application/json" },
  }));

  try {
    const response = await worker.fetch(new Request(
      "https://api.example/v1/crypto/futures/analysis?symbol=BTC_USDT&timeframe=1h&limit=40",
    ));
    assert.equal(response.status, 200);
    const payload = await response.json();
    assert.equal(payload.meta.symbol, "BTC_USDT");
    assert.equal(payload.meta.timeframe, "1h");
    assert.equal(payload.candles.length, 40);
    assert.equal(payload.meta.latestCandle.isClosed, true);
    assert.equal(payload.analysis.candleCountUsed, 40);
    assert.equal(payload.analysis.indicators.values.sma["20"], 129.5);
    assert.equal(payload.analysis.indicators.availability.rsi14, true);
    assert.equal(payload.dataQuality.valid, true);
  } finally {
    mock.restoreAll();
  }
});

test("analysis endpoint preserves multiplier contract symbols", async () => {
  const now = Math.floor(Date.now() / 1000);
  const openTime = Math.floor(now / 60) * 60 - 120;
  const upstreamPayload = {
    success: true,
    code: 0,
    data: {
      time: [openTime, openTime + 60],
      open: [1, 1],
      high: [2, 2],
      low: [0.5, 0.5],
      close: [1.5, 1.5],
      vol: [10, 11],
    },
  };

  mock.method(globalThis, "fetch", async () => new Response(JSON.stringify(upstreamPayload), {
    status: 200,
    headers: { "Content-Type": "application/json" },
  }));

  try {
    const response = await worker.fetch(new Request(
      "https://api.example/v1/crypto/futures/analysis?symbol=1000PEPE_USDT&timeframe=1m&limit=2",
    ));
    assert.equal(response.status, 200);
    const payload = await response.json();
    assert.equal(payload.meta.symbol, "1000PEPE_USDT");
  } finally {
    mock.restoreAll();
  }
});
