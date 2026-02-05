# Metrics

## Introduciton

### Purpose

Provide a metric for scraping.

### Referenes

- [](https://opentelemetry.io/docs/specs/otel/metrics/)
- [Metrics](https://opentelemetry.io/docs/concepts/signals/metrics/)

- [advanced metrics example](https://github.com/open-telemetry/opentelemetry-rust/blob/main/examples/metrics-advanced/src/main.rs)
- [OpenTelemetry prometheus](https://github.com/open-telemetry/opentelemetry-rust/tree/main/opentelemetry-prometheus)

- [](https://github.com/tokio-rs/axum/blob/main/examples/http-proxy/Cargo.toml)
- [](https://hyper.rs/guides/1/upgrading/)

### Usage

- cargo run
- curl localhost:9030
  - `Hello World`
- curl localhost:9030/metrics

```text
# HELP example_http_request_duration_milliseconds The HTTP request latencies in milliseconds.
# TYPE example_http_request_duration_milliseconds histogram
example_http_request_duration_milliseconds_bucket{otel_scope_name="hyper-example",le="0"} 0
...
```
