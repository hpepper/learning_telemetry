# c

- docker run -d --name jaeger --rm -p16686:16686 -p4317:4317 -e COLLECTOR_OTLP_ENABLED=true jaegertracing/all-in-one:latest
- docker run -p 3000:3000 -p 4317:4317 -p 4318:4318 --rm -it grafana/otel-lgtm
- Access the browser via http://localhost:3000
  - admin/admin
- cargo run
- curl http://localhost:8080/rolldice
- See the result at http://localhost:16686

