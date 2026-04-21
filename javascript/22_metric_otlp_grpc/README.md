# Javascript OTLP Metrics over grpc

## Introduction

### Purpose

Test out sending metrics in OTLP over grpc.

### References

- [Get telemetry for your app in less than 5 minutes!](https://opentelemetry.io/docs/languages/js/getting-started/nodejs/)
- [A language-specific implementation of OpenTelemetry in JavaScript](https://opentelemetry.io/docs/languages/js/)

### Vocabulary

- GRPC -
- OTLP -


### Installation

- mkdir 22_metric_otlp_grpc
- cd  22_metric_otlp_grpc
- npm init
- npm install @opentelemetry/sdk-node @opentelemetry/api @opentelemetry/auto-instrumentations-node @opentelemetry/sdk-metrics 
- touch instrumentation.mjs
  - [Code example](https://opentelemetry.io/docs/languages/js/getting-started/nodejs/#setup)
- node --import ./instrumentation.mjs app.js

### Testing it out

- docker run -p 3000:3000 -p 4317:4317 -p 4318:4318 --rm -it grafana/otel-lgtm
  - ports
    - 3000 - Grafana. admin/admin
    - 4317 - Opentelemetry GRPC endpoint
    - 4318 - OpenTelemetry HTTP endpoint
- node --import ./instrumentation.mjs app.js
- OTLP_METRICS_BACKEND_URL=http://localhost:4317 npm start


## Troubleshooting

#### 'total_connections' renamed to 'connections_total'

This is automatic name normalization applied by the OpenTelemetry → Prometheus bridge (inside the collector or Grafana Agent).

Two transformations happen to every Counter:

_total suffix is appended — Prometheus convention requires all counters end with _total.
The leading total_ prefix is stripped — since _total is now the canonical suffix, a total_ prefix is considered redundant and removed to avoid total_connections_total.
So: total_connections → strip total_ prefix → connections → append _total suffix → connections_total.

The idiomatic fix is to name the counter after what is being counted, without total, and let the bridge add _total for you: