/*instrumentation.mjs*/
import { OTLPMetricExporter } from '@opentelemetry/exporter-metrics-otlp-grpc';
import { MeterProvider, PeriodicExportingMetricReader } from '@opentelemetry/sdk-metrics';
import { Resource } from '@opentelemetry/resources';
import { metrics } from '@opentelemetry/api';

function initMetrics() {
  const endpoint = process.env.OTLP_METRICS_BACKEND_URL ?? 'http://localhost:4317';

  const exporter = new OTLPMetricExporter({
    url: endpoint,
    timeoutMillis: 3000,
  });

  const resource = new Resource({
    'service.name': 'api-service',
    'service.version': '0.1.0',
    'service.namespace': 'microservice-simulator',
  });

  const meterProvider = new MeterProvider({
    resource,
    readers: [
      new PeriodicExportingMetricReader({
        exporter,
      }),
    ],
  });

  metrics.setGlobalMeterProvider(meterProvider);
  return endpoint;
}

try {
  const endpoint = initMetrics();
  console.log(`OpenTelemetry metrics initialized - sending metrics to ${endpoint}`);
} catch (err) {
  console.error('Failed to initialize OpenTelemetry metrics:', err);
  console.error('Continuing without metrics...');
}
