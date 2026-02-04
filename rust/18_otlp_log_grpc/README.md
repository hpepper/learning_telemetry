# Sent otlp log via gRPC

## Introduction

### Purpose

Show how to send logs over gRPC to an OpenTelemetry backend and to stdout.

### References

### Vocabulary

### Setting up

- cargo add opentelemetry
- cargo add opentelemetry_log
  - TODO why are things like axum installed with this????
  - TODO 195 sub crates what on earth is going on???
- cargo add opentelemetry-otlp
- cargo add opentelemetry_sdk
- cargo add opentelemetry-appender-log
  - TODO should the serde feature be enabled?
- cargo add opentelemetry-semantic-conventions
- cargo add log
  - TODO what features should be enabled?
    - kv?
    - serde?

To have a backend start the otel-lgtm:

- docker run -p 3000:3000 -p 4317:4317 -p 4318:4318 --rm -it grafana/otel-lgtm
  - ports
    - 3000 - Grafana. admin/admin
    - 4317 - Opentelemetry GRPC endpoint
    - 4318 - OpenTelemetry HTTP endpoint
  - Wait until the  The information is shown: `The OpenTelemetry collector and the Grafana LGTM stack are up and running` with a port summary.
- To see the logs go to explorere -> logs.

## Troubleshooting

### log transfer

#### No logs showing up in the explorerer

A: the app has to wait so the cache can be flushed.

Originially the app just generated the logs and exited.

To fix, added a flush and a wait.