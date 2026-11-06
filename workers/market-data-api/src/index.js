const API_VERSION = "1.0.0";
const UPSTREAM_BASE = "https://api.mexc.com";
const ALLOWED_ORIGINS = new Set([
  "https://doanson44.github.io",
  "http://localhost:8080",
  "http://127.0.0.1:8080",
]);

const TIMEFRAMES = {
  "1m": { interval: "Min1", seconds: 60 },
  "5m": { interval: "Min5", seconds: 300 },
  "15m": { interval: "Min15", seconds: 900 },
  "30m": { interval: "Min30", seconds: 1800 },
  "1h": { interval: "Min60", seconds: 3600 },
  "4h": { interval: "Hour4", seconds: 14400 },
  "8h": { interval: "Hour8", seconds: 28800 },
  "1d": { interval: "Day1", seconds: 86400 },
  "1w": { interval: "Week1", seconds: 604800 },
  "1mo": { interval: "Month1", seconds: 2592000 },
};

const JSON_HEADERS = {
  "Content-Type": "application/json; charset=utf-8",
  "Cache-Control": "public, max-age=5, s-maxage=5",
  "X-Content-Type-Options": "nosniff",
};

export default {
  async fetch(request) {
    const url = new URL(request.url);
    const origin = request.headers.get("Origin");

    if (request.method === "OPTIONS") {
      return handleOptions(origin);
    }

    if (origin && !ALLOWED_ORIGINS.has(origin)) {
      return jsonError("origin_not_allowed", "Origin is not allowed.", 403, origin);
    }

    if (request.method !== "GET") {
      return jsonError("method_not_allowed", "Only GET and OPTIONS are supported.", 405, origin, {
        Allow: "GET, OPTIONS",
      });
    }

    try {
      if (url.pathname === "/health") {
        return jsonResponse({
          status: "ok",
          service: "doanson44-market-data-api",
          apiVersion: API_VERSION,
          timestamp: new Date().toISOString(),
        }, 200, origin, 5);
      }

      if (url.pathname === "/openapi.json") {
        return new Response(OPENAPI_DOCUMENT, {
          status: 200,
          headers: corsHeaders(origin, {
            "Content-Type": "application/json; charset=utf-8",
            "Cache-Control": "public, max-age=300, s-maxage=300",
            "X-Content-Type-Options": "nosniff",
          }),
        });
      }

      if (url.pathname === "/v1/crypto/futures/klines") {
        return await handleKlines(url, origin);
      }

      if (url.pathname === "/v1/crypto/futures/analysis") {
        return await handleAnalysis(url, origin);
      }

      if (url.pathname === "/v1/crypto/futures/ticker") {
        return await handleTicker(url, origin);
      }

      if (url.pathname === "/v1/crypto/futures/funding-rate") {
        return await handleFundingRate(url, origin);
      }

      return jsonError("not_found", "Endpoint not found.", 404, origin);
    } catch (error) {
      return jsonError(
        "internal_error",
        "The market-data service could not complete the request.",
        500,
        origin,
      );
    }
  },
};

async function handleKlines(url, origin) {
  const query = parseKlineQuery(url);
  if (query.error) {
    return jsonError("invalid_query", query.error, 400, origin);
  }

  const upstream = await fetchKlines(query);
  if (upstream.error) {
    return jsonError(upstream.code, upstream.error, upstream.status, origin);
  }

  const payload = buildMarketPayload(query, upstream.candles);
  return jsonResponse(payload, 200, origin, 15);
}

async function handleAnalysis(url, origin) {
  const query = parseKlineQuery(url);
  if (query.error) {
    return jsonError("invalid_query", query.error, 400, origin);
  }

  const upstream = await fetchKlines(query);
  if (upstream.error) {
    return jsonError(upstream.code, upstream.error, upstream.status, origin);
  }

  const market = buildMarketPayload(query, upstream.candles);
  const indicators = calculateIndicators(upstream.candles);
  return jsonResponse({
    ...market,
    analysis: {
      engine: "market-data-api-indicators",
      engineVersion: API_VERSION,
      methodology: "Standard technical indicator calculations over the returned OHLCV candles.",
      indicators,
      marketStructure: {
        status: "not_calculated",
        reason: "BOS, CHoCH, swing structure, support/resistance, and pattern detection are not part of this API version.",
      },
    },
  }, 200, origin, 15);
}

async function handleTicker(url, origin) {
  const symbol = parseSymbol(url.searchParams.get("symbol"));
  if (!symbol) {
    return jsonError("invalid_symbol", "A valid MEXC Futures symbol is required, e.g. BTC_USDT or 1000PEPE_USDT.", 400, origin);
  }

  const target = new URL("/api/v1/contract/ticker", UPSTREAM_BASE);
  target.searchParams.set("symbol", symbol);
  const response = await fetchUpstream(target);
  if (response.error) {
    return jsonError(response.code, response.error, response.status, origin);
  }

  const data = response.payload && response.payload.data;
  const ticker = Array.isArray(data) ? data.find((item) => item.symbol === symbol) : data;
  if (!ticker || ticker.symbol !== symbol) {
    return jsonError("upstream_data_missing", "MEXC did not return ticker data for the requested symbol.", 502, origin);
  }

  return jsonResponse({
    schemaVersion: "1.0",
    meta: {
      source: "MEXC Futures",
      sourceEndpoint: "/api/v1/contract/ticker",
      symbol,
      retrievedAt: new Date().toISOString(),
      sourceTimestamp: toIsoTimestamp(ticker.timestamp),
      freshness: "upstream_timestamp",
    },
    ticker: {
      symbol,
      lastPrice: finiteOrNull(ticker.lastPrice),
      bidPrice: finiteOrNull(ticker.bid1),
      askPrice: finiteOrNull(ticker.ask1),
      fairPrice: finiteOrNull(ticker.fairPrice),
      indexPrice: finiteOrNull(ticker.indexPrice),
      fundingRate: finiteOrNull(ticker.fundingRate),
      volume24h: finiteOrNull(ticker.volume24),
      openInterest: finiteOrNull(ticker.holdVol),
      high24h: finiteOrNull(ticker.high24Price),
      low24h: finiteOrNull(ticker.lower24Price),
      changeRate24h: finiteOrNull(ticker.riseFallRate),
    },
  }, 200, origin, 3);
}

async function handleFundingRate(url, origin) {
  const symbol = parseSymbol(url.searchParams.get("symbol"));
  if (!symbol) {
    return jsonError("invalid_symbol", "A valid MEXC Futures symbol is required, e.g. BTC_USDT or 1000PEPE_USDT.", 400, origin);
  }

  const target = new URL("/api/v1/contract/funding_rate/" + encodeURIComponent(symbol), UPSTREAM_BASE);
  const response = await fetchUpstream(target);
  if (response.error) {
    return jsonError(response.code, response.error, response.status, origin);
  }

  const data = response.payload && response.payload.data;
  if (!data || data.symbol !== symbol) {
    return jsonError("upstream_data_missing", "MEXC did not return funding-rate data for the requested symbol.", 502, origin);
  }

  return jsonResponse({
    schemaVersion: "1.0",
    meta: {
      source: "MEXC Futures",
      sourceEndpoint: "/api/v1/contract/funding_rate/{symbol}",
      symbol,
      retrievedAt: new Date().toISOString(),
      sourceTimestamp: toIsoTimestamp(data.timestamp),
    },
    funding: {
      symbol,
      fundingRate: finiteOrNull(data.fundingRate),
      minFundingRate: finiteOrNull(data.minFundingRate),
      maxFundingRate: finiteOrNull(data.maxFundingRate),
      settlementIntervalHours: finiteOrNull(data.collectCycle),
      nextSettlementAt: toIsoTimestamp(data.nextSettleTime),
    },
  }, 200, origin, 15);
}

function parseKlineQuery(url) {
  const symbol = parseSymbol(url.searchParams.get("symbol"));
  if (!symbol) {
    return { error: "A valid MEXC Futures symbol is required, e.g. BTC_USDT or 1000PEPE_USDT." };
  }

  const timeframe = (url.searchParams.get("timeframe") || "1h").toLowerCase();
  const interval = TIMEFRAMES[timeframe];
  if (!interval) {
    return { error: "Unsupported timeframe. Supported values: " + Object.keys(TIMEFRAMES).join(", ") + "." };
  }

  const limitValue = url.searchParams.get("limit");
  const limit = limitValue === null ? 500 : Number(limitValue);
  if (!Number.isInteger(limit) || limit < 2 || limit > 2000) {
    return { error: "limit must be an integer between 2 and 2000." };
  }

  const now = Math.floor(Date.now() / 1000);
  const endValue = url.searchParams.get("end");
  const startValue = url.searchParams.get("start");
  const end = endValue === null ? now : Number(endValue);
  const start = startValue === null ? end - interval.seconds * limit : Number(startValue);

  if (!Number.isSafeInteger(start) || !Number.isSafeInteger(end) || start <= 0 || end <= start) {
    return { error: "start and end must be valid Unix timestamps in seconds, and end must be greater than start." };
  }

  if (end - start > interval.seconds * 2000) {
    return { error: "The requested time range exceeds MEXC's 2000-candle response limit." };
  }

  return { symbol, timeframe, interval, limit, start, end };
}

function parseSymbol(value) {
  if (typeof value !== "string") return null;
  const symbol = value.trim().toUpperCase();
  return /^[A-Z0-9]{1,30}_USDT$/.test(symbol) ? symbol : null;
}

async function fetchKlines(query) {
  const target = new URL(
    "/api/v1/contract/kline/" + encodeURIComponent(query.symbol),
    UPSTREAM_BASE,
  );
  target.searchParams.set("interval", query.interval.interval);
  target.searchParams.set("start", String(query.start));
  target.searchParams.set("end", String(query.end));

  const response = await fetchUpstream(target);
  if (response.error) return response;

  const payload = response.payload;
  if (!payload || payload.success !== true || !payload.data) {
    return {
      error: "MEXC returned an unsuccessful or malformed K-line response.",
      code: "upstream_invalid_response",
      status: 502,
    };
  }

  const data = payload.data;
  const arrays = ["time", "open", "high", "low", "close", "vol"];
  const count = Array.isArray(data.time) ? data.time.length : -1;
  if (count < 1 || arrays.some((key) => !Array.isArray(data[key]) || data[key].length !== count)) {
    return {
      error: "MEXC returned inconsistent K-line arrays.",
      code: "upstream_invalid_data",
      status: 502,
    };
  }

  const candles = [];
  for (let index = 0; index < count; index += 1) {
    const candle = {
      timestamp: Number(data.time[index]),
      open: Number(data.open[index]),
      high: Number(data.high[index]),
      low: Number(data.low[index]),
      close: Number(data.close[index]),
      volume: Number(data.vol[index]),
      amount: Array.isArray(data.amount) ? finiteOrNull(data.amount[index]) : null,
    };

    if (!Number.isSafeInteger(candle.timestamp) ||
        ![candle.open, candle.high, candle.low, candle.close, candle.volume].every(Number.isFinite) ||
        candle.open <= 0 || candle.high <= 0 || candle.low <= 0 || candle.close <= 0 ||
        candle.volume < 0 || candle.high < Math.max(candle.open, candle.close, candle.low) ||
        candle.low > Math.min(candle.open, candle.close, candle.high)) {
      return {
        error: "MEXC returned an invalid OHLCV candle at index " + index + ".",
        code: "upstream_invalid_candle",
        status: 502,
      };
    }
    candles.push(candle);
  }

  candles.sort((left, right) => left.timestamp - right.timestamp);
  const unique = [];
  for (const candle of candles) {
    if (unique.length && unique[unique.length - 1].timestamp === candle.timestamp) {
      unique[unique.length - 1] = candle;
    } else {
      unique.push(candle);
    }
  }

  return { candles: unique };
}

async function fetchUpstream(url) {
  let response;
  try {
    response = await fetch(url.toString(), {
      method: "GET",
      headers: { Accept: "application/json" },
    });
  } catch (error) {
    return {
      error: "Unable to reach the MEXC public market-data API.",
      code: "upstream_unavailable",
      status: 502,
    };
  }

  if (!response.ok) {
    return {
      error: "MEXC returned HTTP " + response.status + ".",
      code: "upstream_http_error",
      status: 502,
    };
  }

  let payload;
  try {
    payload = await response.json();
  } catch (error) {
    return {
      error: "MEXC returned invalid JSON.",
      code: "upstream_invalid_json",
      status: 502,
    };
  }

  return { payload };
}

function buildMarketPayload(query, candles) {
  const now = Math.floor(Date.now() / 1000);
  const last = candles[candles.length - 1];
  const gaps = [];
  for (let index = 1; index < candles.length; index += 1) {
    const delta = candles[index].timestamp - candles[index - 1].timestamp;
    if (delta > query.interval.seconds * 1.5) {
      gaps.push({
        from: candles[index - 1].timestamp,
        to: candles[index].timestamp,
        missingIntervals: Math.max(1, Math.round(delta / query.interval.seconds) - 1),
      });
    }
  }

  return {
    schemaVersion: "1.0",
    meta: {
      source: "MEXC Futures",
      sourceEndpoint: "/api/v1/contract/kline/{symbol}",
      symbol: query.symbol,
      timeframe: query.timeframe,
      interval: query.interval.interval,
      timezone: "UTC",
      retrievedAt: new Date().toISOString(),
      requestedRange: {
        start: query.start,
        end: query.end,
      },
      candleCount: candles.length,
      requestedLimit: query.limit,
      latestCandle: {
        openTime: last.timestamp,
        closeTime: last.timestamp + query.interval.seconds,
        isClosed: now >= last.timestamp + query.interval.seconds,
      },
    },
    candles,
    dataQuality: {
      valid: gaps.length === 0,
      duplicateCandlesRemoved: 0,
      missingIntervals: gaps,
      issues: gaps.length ? ["missing_candle_intervals_detected"] : [],
    },
  };
}

function calculateIndicators(candles) {
  const closes = candles.map((candle) => candle.close);
  const highs = candles.map((candle) => candle.high);
  const lows = candles.map((candle) => candle.low);
  const volumes = candles.map((candle) => candle.volume);
  const latest = candles.length - 1;

  const sma = {};
  for (const period of [20, 50, 100, 200]) {
    sma[String(period)] = lastValue(smaSeries(closes, period));
  }

  const ema = {};
  for (const period of [9, 20, 50, 200]) {
    ema[String(period)] = lastValue(emaSeries(closes, period));
  }

  const rsi = lastValue(rsiSeries(closes, 14));
  const macd = macdLatest(closes, 12, 26, 9);
  const stochastic = stochasticLatest(highs, lows, closes, 14, 3, 3);
  const atr = lastValue(atrSeries(candles, 14));
  const bollinger = bollingerLatest(closes, 20, 2);
  const adx = adxLatest(candles, 14);
  const volumeSma20 = lastValue(smaSeries(volumes, 20));
  const obv = obvLatest(closes, volumes);

  return {
    periods: {
      movingAverage: [9, 20, 50, 100, 200],
      rsi: 14,
      macd: { fast: 12, slow: 26, signal: 9 },
      stochastic: { k: 14, d: 3, smooth: 3 },
      atr: 14,
      bollingerBands: { period: 20, standardDeviations: 2 },
      adx: 14,
      volumeAverage: [20],
    },
    values: {
      sma,
      ema,
      rsi14: rsi,
      macd,
      stochastic,
      atr14: atr,
      atrPercent: atr !== null && closes[latest] > 0 ? (atr / closes[latest]) * 100 : null,
      bollingerBands: bollinger,
      adx14: adx,
      volumeSma20,
      obv,
    },
    availability: {
      sma20: closes.length >= 20,
      sma50: closes.length >= 50,
      sma100: closes.length >= 100,
      sma200: closes.length >= 200,
      rsi14: closes.length >= 15,
      macd: closes.length >= 35,
      stochastic: closes.length >= 19,
      atr14: closes.length >= 15,
      bollingerBands: closes.length >= 20,
      adx14: closes.length >= 28,
      volumeSma20: volumes.length >= 20,
      obv: closes.length >= 1,
    },
  };
}

function smaSeries(values, period) {
  const output = Array(values.length).fill(null);
  if (values.length < period) return output;
  let sum = 0;
  for (let index = 0; index < values.length; index += 1) {
    sum += values[index];
    if (index >= period) sum -= values[index - period];
    if (index >= period - 1) output[index] = sum / period;
  }
  return output;
}

function emaSeries(values, period) {
  const output = Array(values.length).fill(null);
  if (values.length < period) return output;
  const multiplier = 2 / (period + 1);
  let previous = values.slice(0, period).reduce((sum, value) => sum + value, 0) / period;
  output[period - 1] = previous;
  for (let index = period; index < values.length; index += 1) {
    previous = (values[index] - previous) * multiplier + previous;
    output[index] = previous;
  }
  return output;
}

function rsiSeries(values, period) {
  const output = Array(values.length).fill(null);
  if (values.length <= period) return output;
  let gain = 0;
  let loss = 0;
  for (let index = 1; index <= period; index += 1) {
    const change = values[index] - values[index - 1];
    gain += Math.max(change, 0);
    loss += Math.max(-change, 0);
  }
  let averageGain = gain / period;
  let averageLoss = loss / period;
  output[period] = rsiValue(averageGain, averageLoss);
  for (let index = period + 1; index < values.length; index += 1) {
    const change = values[index] - values[index - 1];
    averageGain = ((averageGain * (period - 1)) + Math.max(change, 0)) / period;
    averageLoss = ((averageLoss * (period - 1)) + Math.max(-change, 0)) / period;
    output[index] = rsiValue(averageGain, averageLoss);
  }
  return output;
}

function rsiValue(gain, loss) {
  if (loss === 0) return gain === 0 ? 50 : 100;
  return 100 - (100 / (1 + gain / loss));
}

function macdLatest(values, fastPeriod, slowPeriod, signalPeriod) {
  const fast = emaSeries(values, fastPeriod);
  const slow = emaSeries(values, slowPeriod);
  const line = values.map((_, index) =>
    fast[index] === null || slow[index] === null ? null : fast[index] - slow[index]
  );
  const firstValid = line.findIndex((value) => value !== null);
  if (firstValid < 0) return { line: null, signal: null, histogram: null };
  const compact = line.slice(firstValid);
  const signal = emaSeries(compact, signalPeriod);
  const index = compact.length - 1;
  const macdLine = line[line.length - 1];
  const signalLine = signal[index];
  return {
    line: finiteOrNull(macdLine),
    signal: finiteOrNull(signalLine),
    histogram: macdLine !== null && signalLine !== null ? macdLine - signalLine : null,
  };
}

function stochasticLatest(highs, lows, closes, kPeriod, dPeriod, smooth) {
  if (closes.length < kPeriod + smooth + dPeriod - 2) {
    return { k: null, d: null };
  }
  const rawK = Array(closes.length).fill(null);
  for (let index = kPeriod - 1; index < closes.length; index += 1) {
    const highest = Math.max(...highs.slice(index - kPeriod + 1, index + 1));
    const lowest = Math.min(...lows.slice(index - kPeriod + 1, index + 1));
    rawK[index] = highest === lowest ? 50 : ((closes[index] - lowest) / (highest - lowest)) * 100;
  }
  const smoothK = movingAverageNullable(rawK, smooth);
  const d = movingAverageNullable(smoothK, dPeriod);
  return {
    k: lastValue(smoothK),
    d: lastValue(d),
  };
}

function movingAverageNullable(values, period) {
  const output = Array(values.length).fill(null);
  for (let index = period - 1; index < values.length; index += 1) {
    const window = values.slice(index - period + 1, index + 1);
    if (window.every((value) => value !== null)) {
      output[index] = window.reduce((sum, value) => sum + value, 0) / period;
    }
  }
  return output;
}

function trueRangeSeries(candles) {
  return candles.map((candle, index) => {
    if (index === 0) return candle.high - candle.low;
    const previousClose = candles[index - 1].close;
    return Math.max(
      candle.high - candle.low,
      Math.abs(candle.high - previousClose),
      Math.abs(candle.low - previousClose),
    );
  });
}

function wilderSeries(values, period) {
  const output = Array(values.length).fill(null);
  if (values.length < period) return output;
  let previous = values.slice(0, period).reduce((sum, value) => sum + value, 0) / period;
  output[period - 1] = previous;
  for (let index = period; index < values.length; index += 1) {
    previous = ((previous * (period - 1)) + values[index]) / period;
    output[index] = previous;
  }
  return output;
}

function atrSeries(candles, period) {
  return wilderSeries(trueRangeSeries(candles), period);
}

function bollingerLatest(values, period, standardDeviations) {
  if (values.length < period) {
    return { middle: null, upper: null, lower: null, bandwidthPercent: null };
  }
  const window = values.slice(-period);
  const mean = window.reduce((sum, value) => sum + value, 0) / period;
  const variance = window.reduce((sum, value) => sum + ((value - mean) ** 2), 0) / period;
  const deviation = Math.sqrt(variance);
  const upper = mean + standardDeviations * deviation;
  const lower = mean - standardDeviations * deviation;
  return {
    middle: mean,
    upper,
    lower,
    bandwidthPercent: mean !== 0 ? ((upper - lower) / mean) * 100 : null,
  };
}

function adxLatest(candles, period) {
  if (candles.length < (period * 2)) return null;
  const plusDm = Array(candles.length).fill(0);
  const minusDm = Array(candles.length).fill(0);
  for (let index = 1; index < candles.length; index += 1) {
    const up = candles[index].high - candles[index - 1].high;
    const down = candles[index - 1].low - candles[index].low;
    plusDm[index] = up > down && up > 0 ? up : 0;
    minusDm[index] = down > up && down > 0 ? down : 0;
  }

  const tr = trueRangeSeries(candles);
  const smoothedTr = wilderSeries(tr, period);
  const smoothedPlus = wilderSeries(plusDm, period);
  const smoothedMinus = wilderSeries(minusDm, period);
  const dx = Array(candles.length).fill(null);

  for (let index = period - 1; index < candles.length; index += 1) {
    if (!smoothedTr[index] || smoothedTr[index] <= 0) continue;
    const plusDi = (smoothedPlus[index] / smoothedTr[index]) * 100;
    const minusDi = (smoothedMinus[index] / smoothedTr[index]) * 100;
    const denominator = plusDi + minusDi;
    dx[index] = denominator === 0 ? 0 : (Math.abs(plusDi - minusDi) / denominator) * 100;
  }

  const firstDx = dx.findIndex((value) => value !== null);
  if (firstDx < 0) return null;
  const adxValues = wilderSeries(dx.slice(firstDx).map((value) => value === null ? 0 : value), period);
  return lastValue(adxValues);
}

function obvLatest(closes, volumes) {
  if (!closes.length) return null;
  let obv = volumes[0];
  for (let index = 1; index < closes.length; index += 1) {
    if (closes[index] > closes[index - 1]) obv += volumes[index];
    else if (closes[index] < closes[index - 1]) obv -= volumes[index];
  }
  return finiteOrNull(obv);
}

function lastValue(values) {
  if (!values.length) return null;
  return finiteOrNull(values[values.length - 1]);
}

function finiteOrNull(value) {
  const number = Number(value);
  return Number.isFinite(number) ? number : null;
}

function toIsoTimestamp(value) {
  const number = Number(value);
  if (!Number.isFinite(number) || number <= 0) return null;
  const milliseconds = number < 100000000000 ? number * 1000 : number;
  const date = new Date(milliseconds);
  return Number.isNaN(date.getTime()) ? null : date.toISOString();
}

function corsHeaders(origin, extra = {}) {
  const headers = new Headers({ ...JSON_HEADERS, ...extra });
  if (origin && ALLOWED_ORIGINS.has(origin)) {
    headers.set("Access-Control-Allow-Origin", origin);
    headers.set("Vary", "Origin");
  }
  return headers;
}

function handleOptions(origin) {
  if (!origin || !ALLOWED_ORIGINS.has(origin)) {
    return new Response(null, { status: 403 });
  }
  return new Response(null, {
    status: 204,
    headers: {
      "Access-Control-Allow-Origin": origin,
      "Access-Control-Allow-Methods": "GET, OPTIONS",
      "Access-Control-Allow-Headers": "Accept, Content-Type",
      "Access-Control-Max-Age": "86400",
      Vary: "Origin",
    },
  });
}

function jsonResponse(body, status, origin, cacheSeconds) {
  const headers = corsHeaders(origin, {
    "Cache-Control": "public, max-age=" + cacheSeconds + ", s-maxage=" + cacheSeconds,
  });
  return new Response(JSON.stringify(body), { status, headers });
}

function jsonError(code, message, status, origin, extraHeaders = {}) {
  return new Response(JSON.stringify({
    schemaVersion: "1.0",
    error: { code, message },
    timestamp: new Date().toISOString(),
  }), {
    status,
    headers: corsHeaders(origin, extraHeaders),
  });
}

const OPENAPI_DOCUMENT = JSON.stringify({
  openapi: "3.1.0",
  info: {
    title: "doanson44 Market Data API",
    version: API_VERSION,
    description: "Read-only normalized MEXC Futures public market data and technical indicators. No account, order, or credential operations are exposed.",
  },
  servers: [{ url: "https://doanson44-market-data-api.<workers-subdomain>.workers.dev" }],
  paths: {
    "/health": {
      get: {
        summary: "Health check",
        responses: { "200": { description: "Service health" } },
      },
    },
    "/v1/crypto/futures/klines": {
      get: {
        summary: "Get normalized Futures OHLCV candles",
        parameters: [
          { name: "symbol", in: "query", required: true, schema: { type: "string", example: "BTC_USDT" } },
          { name: "timeframe", in: "query", schema: { type: "string", enum: Object.keys(TIMEFRAMES), default: "1h" } },
          { name: "limit", in: "query", schema: { type: "integer", minimum: 2, maximum: 2000, default: 500 } },
          { name: "start", in: "query", description: "Unix timestamp in seconds", schema: { type: "integer" } },
          { name: "end", in: "query", description: "Unix timestamp in seconds", schema: { type: "integer" } },
        ],
        responses: { "200": { description: "Normalized OHLCV candles and data-quality metadata" }, "400": { description: "Invalid query" }, "502": { description: "Upstream error" } },
      },
    },
    "/v1/crypto/futures/analysis": {
      get: {
        summary: "Get OHLCV plus calculated technical indicators",
        parameters: [
          { name: "symbol", in: "query", required: true, schema: { type: "string", example: "BTC_USDT" } },
          { name: "timeframe", in: "query", schema: { type: "string", enum: Object.keys(TIMEFRAMES), default: "1h" } },
          { name: "limit", in: "query", schema: { type: "integer", minimum: 2, maximum: 2000, default: 500 } },
        ],
        responses: { "200": { description: "OHLCV, indicator values, availability flags, and data-quality metadata" } },
      },
    },
    "/v1/crypto/futures/ticker": {
      get: {
        summary: "Get a Futures ticker snapshot",
        parameters: [{ name: "symbol", in: "query", required: true, schema: { type: "string", example: "BTC_USDT" } }],
        responses: { "200": { description: "Ticker snapshot with source timestamp" } },
      },
    },
    "/v1/crypto/futures/funding-rate": {
      get: {
        summary: "Get a Futures funding-rate snapshot",
        parameters: [{ name: "symbol", in: "query", required: true, schema: { type: "string", example: "BTC_USDT" } }],
        responses: { "200": { description: "Funding rate and settlement metadata" } },
      },
    },
  },
});

