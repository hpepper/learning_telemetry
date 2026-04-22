# Using opentelemetry from rust


https://opentelemetry.io/docs/languages/rust/

https://github.com/open-telemetry/opentelemetry-rust


LOGS

Logs often contain detailed debugging/diagnostic info, such as inputs to an operation, the result of the operation, and any supporting metadata for that operation.


## otel-lgtm container


### Change opentelemetry to allow out_of_order_time_window datapoint

- grab the existing config from the running container so you don't break anything else:
  - docker exec <container_name> cat /otel-lgtm/prometheus.yaml > prometheus.yaml
- add the storage block to your local prometheus.yml:
```yaml
storage:
  tsdb:
    out_of_order_time_window: 60d
```
- mount it when you start the container. If you're using docker run:
  - docker run -v $(pwd)/prometheus.yaml:/otel-lgtm/prometheus.yaml -p 3000:3000 -p 4317:4317 -p 4318:4318 --rm -it grafana/otel-lgtm
