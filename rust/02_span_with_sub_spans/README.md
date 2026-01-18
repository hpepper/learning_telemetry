# Multiple calls

This example shows how to have all sub span event under the same top span for a single function.

- [](https://opentelemetry.io/docs/instrumentation/rust/getting-started/)
- [](https://docs.rs/opentelemetry-jaeger/latest/opentelemetry_jaeger/)

- docker run -d --name jaeger --rm -p16686:16686 -p4317:4317 -e COLLECTOR_OTLP_ENABLED=true jaegertracing/all-in-one:latest
- cargo run
- curl http://localhost:8080/rolldice
- See the result at http://localhost:16686