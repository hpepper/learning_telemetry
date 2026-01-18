# First http trace

- [](https://opentelemetry.io/docs/instrumentation/rust/getting-started/)
- [](https://docs.rs/opentelemetry-jaeger/latest/opentelemetry_jaeger/)

* Port
 - 4317 - grpc
 - 4318 - http

- docker run -d --name jaeger --rm -e COLLECTOR_ZIPKIN_HOST_PORT=:9411 -e COLLECTOR_OTLP_ENABLED=true -p 6831:6831/udp -p 6832:6832/udp -p 5778:5778 -p 16686:16686 -p 4317:4317 -p 4318:4318 -p 14250:14250 -p 14268:14268 -p 14269:14269 -p 9411:9411 jaegertracing/all-in-one:latest

- docker run -d --name jaeger --rm -p16686:16686 -p4317:4317 -e COLLECTOR_OTLP_ENABLED=true jaegertracing/all-in-one:latest
