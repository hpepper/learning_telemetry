# Rust learning telemetry

## Introduction

### Purpose

### References

- [OpenTelemetry Rust](https://github.com/open-telemetry/opentelemetry-rust#overview-of-crates)
- [Crate opentelemetry](https://docs.rs/opentelemetry/latest/opentelemetry/)
- [tokio-rs / tracing](https://github.com/tokio-rs/tracing) - What is this used for?

### Vocabulary

- Exporter - sends data to backend, e.g. opentelemetry-otlp
- otlp - OpenTelemetry Protocol.

### Crates for OpenTelemtry

- Required crates
  - [opentelemetry](https://crates.io/crates/opentelemetry) - serves as a facade or no-op implementation, meaning it defines the traits for instrumentation but does not itself implement the processing or exporting of telemetry data.
    - Actual implementation are handled by
      - opentelemetry-sdk crate
      - and various exporter crates such as opentelemetry-otlp
  - [opentelemetry-sdk](https://crates.io/crates/opentelemetry-sdk) - the official SDK implemented by OpenTelemetry itself.
  - [opentelemetry-otlp](https://crates.io/crates/opentelemetry-otlp) - Exports telemetry (logs, metrics and traces) in the OTLP format to an endpoint accepting OTLP
- Additional crates
  - [opentelemetry-stdout](https://crates.io/crates/opentelemetry-stdout) - Prints telemetry to stdout, primarily used for learning/debugging purposes.
  - []()
  - []()
  
