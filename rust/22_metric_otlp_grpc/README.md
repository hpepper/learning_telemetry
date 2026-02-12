# Send metrics using OLTP via gRPC

## Introduction

### Purpose

Show how to set up metrics transmitting metrics with otlp over gRPC.

### References

- https://github.com/open-telemetry/opentelemetry-rust/blob/main/opentelemetry-otlp/examples/basic-otlp/src/main.rs
- https://github.com/open-telemetry/opentelemetry-rust/blob/main/examples/metrics-basic/src/main.rs
- [Metrics specifications](https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/metrics/sdk.md)


### Vocabulary

- opentelemetry-otlp - the exporter part
- opentelemetry-sdk - the data gather thing.
  - https://crates.io/crates/opentelemetry-sdk
  - 

### Prep build env

`cargo info opentelemetry-otlp`
This didn't show grpc-tonic as a feature.

- cargo add opentelemetry
- cargo add opentelemetry-otlp --features grpc-tonic,metrics
- cargo add opentelemetry_sdk

### Testing it out

- docker run -p 3000:3000 -p 4317:4317 -p 4318:4318 --rm -it grafana/otel-lgtm
  - ports
    - 3000 - Grafana. admin/admin
    - 4317 - Opentelemetry GRPC endpoint
    - 4318 - OpenTelemetry HTTP endpoint
- OTLP_METRICS_BACKEND_URL=http://localhost:4317 cargo run
