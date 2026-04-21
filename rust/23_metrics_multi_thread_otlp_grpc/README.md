# Have a commong metric counter sent over grpc

## Introduction

### Purpose

Describe how to use a single metric counter to count all connections.

TODO

- Have a common/global counter that counts the total number of connections.
- Have a common/global gauge? the holds the number of current connections.

### References

- [Metrics](https://opentelemetry.io/docs/concepts/signals/metrics/)
- [Language API/SDK status](https://opentelemetry.io/status/)
- [Metrics SDK](https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/metrics/sdk.md)
- [opentelemetry rust](https://opentelemetry.io/docs/languages/rust/)
  - [OpenTelemetry Rust SDK](https://crates.io/crates/opentelemetry-sdk)
- [](https://opentelemetry.io/docs/)
- [Beyond the good first issue - How to make your contributions sustainable](https://opentelemetry.io/blog/2026/alternative-approaches-to-contributing/)

### Vocabulary

- [Aggregation](https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/metrics/sdk.md#aggregation) - TODO explain
- [Instrumentation Scope](https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/common/instrumentation-scope.md) - A logical unit of software with which the emitted telemetry can be associated.
- Metric kinds -
  - [Counter](https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/metrics/api.md#counter) - supports non-negative increments.
    - [Asynchronous Counter](https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/metrics/api.md#asynchronous-counter)
    - UpDownCounter
  - [Gauge](https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/metrics/api.md#gauge) - used to record non-additive value(s).
    - [Asynchronous Gauge](https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/metrics/api.md#asynchronous-gauge)
  - [Histogram](https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/metrics/api.md#histogram) - used to report arbitrary values that are likely to be statistically meaningful.
- [Resource](https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/resource/sdk.md) - an immutable representation of the observed entity for which telemetry is being produced, expressed as Attributes.
- Resource detector - TODO what is this???

### Prep build env

- cargo add opentelemetry
- cargo add opentelemetry-otlp --features grpc-tonic,metrics
- cargo add opentelemetry_sdk
- cargo add rand
- cargo add tokio --features macros,rt-multi-thread
- cargo add axum
- cargo add serde --features derive
- cargo add serde_json

### Testing it out

- docker run -p 3000:3000 -p 4317:4317 -p 4318:4318 --rm -it grafana/otel-lgtm
  - ports
    - 3000 - Grafana. admin/admin
    - 4317 - Opentelemetry GRPC endpoint
    - 4318 - OpenTelemetry HTTP endpoint
- OTLP_METRICS_BACKEND_URL=http://localhost:4317 cargo run


#### calling the app

```bash
curl http://localhost:8080/health
curl "http://localhost:8080/d6"
curl "http://localhost:8080/d6?count=3"
curl -i "http://localhost:8080/d6?count=0"
curl "http://localhost:8080/d8?count=5"
```


## Next test

From claude

At that scale, the current pattern has a real performance problem: every request calls global::meter() and .build(), which does internal registry lookups (with lock acquisition) on every single call. At 10k RPS × 100 paths that's significant unnecessary overhead.

The fix is to create instruments once at startup and inject them via Axum's State extractor:

```rust
use std::sync::Arc;

struct AppState {
    roll_d6_calls: opentelemetry::metrics::Counter<u64>,
    roll_d8_calls: opentelemetry::metrics::Counter<u64>,
    active_requests: opentelemetry::metrics::UpDownCounter<i64>,
    // ... one field per instrument
}
```


Then at startup:

```rust
let meter = global::meter("MetricMultiThreadSpike");
let state = Arc::new(AppState {
    roll_d6_calls: meter.u64_counter("roll_d6_calls").build(),
    roll_d8_calls: meter.u64_counter("roll_d8_calls").build(),
    active_requests: meter.i64_up_down_counter("active_requests").build(),
});

let app = Router::new()
    .route("/d6", get(roll_d6))
    .route("/d8", get(roll_d8))
    .with_state(state);
```

And in handlers:

```rust
async fn roll_d6(
    State(state): State<Arc<AppState>>,
    Query(params): Query<DiceQuery>,
) -> ... {
    state.roll_d6_calls.add(1, &[]);
    state.active_requests.add(1, &[KeyValue::new("handler", "roll_d6")]);
    // ...
}
```

Why this matters at scale:

Counter<u64> and UpDownCounter<i64> are Clone + cheap to share via Arc — the handles are just thin wrappers
The hot path becomes only the AtomicI64::fetch_add() with zero SDK registry lookups
Lock contention on the attribute map effectively disappears since the slots are already warm from startup
Other things to watch at 10k RPS:

The periodic OTLP exporter runs on a background thread — make sure its export interval/timeout is tuned so it doesn't back up
With 100 paths × N attribute combos, watch total number of active time series — the SDK holds all of them in memory between exports.



#### async thinging

An asynchronous (observable) UpDownCounter works differently — instead of you calling add(), you register a callback that the SDK calls at export time to pull the current value:

```rust￼
let active = Arc::new(AtomicI64::new(0));let active_clone = Arc::clone(&active);meter    .i64_observable_up_down_counter("active_requests")    .with_description("Number of requests currently being executed")    .with_callback(move |observer| {        observer.observe(active_clone.load(Ordering::Relaxed), &[]);    })    .build();
```

Then in your handlers you manually update the AtomicI64:

```rust￼
active.fetch_add(1, Ordering::Relaxed);  // on entryactive.fetch_add(-1, Ordering::Relaxed); // on exit
```

Async vs Synchronous UpDownCounter — which fits here?

| --------------------- | ---------------------------- | --------- | 
|                       | Sync UpDownCounter           | Async ObservableUpDownCounter |
| How value is reported	| Every add() call is recorded | SDK polls your callback at export interval |
| Updates per second    | All 10k RPS tracked | Only the value at poll time is seen |
| Overhead per request | One AtomicI64::fetch_add + map lookup | One AtomicI64::fetch_add (just yours, no SDK involvement) |
| Good for | Deltas, counters, things that accumulate | Current state you already track yourself |

For active_requests specifically

The async variant is arguably a better semantic fit — you only care about the current value at export time, not every individual increment/decrement. The SDK doesn't need to see every transition.

However, with the Arc<AppState> approach from the previous answer, the sync version's overhead is already just one fetch_add per call — so the practical difference is negligible at 10k RPS.

The async version becomes clearly preferable when you already have the value somewhere (e.g., a connection pool that tracks its own size) and you just want to expose it — no point calling add() when you can just read the existing atomic.


TODO calculate what is the cost of doing the lookup for the company app.
