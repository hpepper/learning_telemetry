use opentelemetry::{global, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{Resource, metrics::SdkMeterProvider};

/**
 * This 
 *   - initializes the otlp grpc connection to the backend
 *   - map the metric api(provider) to the exporter
 *   - stores the metric provider in the global registry so that it can be used by the rest of the application
 */
fn init_metrics() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let endpoint = std::env::var("OTLP_METRICS_BACKEND_URL")
        .unwrap_or_else(|_| "http://localhost:4317".to_string());

    let otlp_exporter = opentelemetry_otlp::MetricExporter::builder()
        .with_tonic()
        .with_endpoint(endpoint)
        .with_timeout(std::time::Duration::from_secs(3))
        .build()?;

    let resource = Resource::builder_empty()
        .with_attribute(KeyValue::new("service.name", "api-service"))
        .with_attribute(KeyValue::new("service.version", "0.1.0"))
        .with_attribute(KeyValue::new("service.namespace", "microservice-simulator"))
        .build();

    let matrics_provider = SdkMeterProvider::builder()
        .with_periodic_exporter(otlp_exporter)
        .with_resource(resource)
        .build();

    global::set_meter_provider(matrics_provider);
    Ok(())
}

// It seems metrics requires tokio, possibly because the metrics are sent every 60 seconds?

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    if let Err(err) = init_metrics() {
        eprintln!("Failed to initialize OpenTelemetry metrics: {}", err);
        eprintln!("Continuing without metrics...");
    } else {
        // TODO actually get this information from the metrics provider, not count on this being the same code as in init_metrics()
        let endpoint = std::env::var("OTLP_METRICS_BACKEND_URL")
            .unwrap_or_else(|_| "http://localhost:4317".to_string());
        println!(
            "OpenTelemetry metrics initialized - sending metrics to {}",
            endpoint
        );
    }

    // https://github.com/open-telemetry/opentelemetry-rust/blob/main/examples/metrics-advanced/src/main.rs
    // https://github.com/open-telemetry/opentelemetry-rust/blob/main/examples/metrics-basic/src/main.rs
    let meter = global::meter("mylibraryname");
    let counter = meter
        .u64_counter("total_connections")
        .with_description("Total connections over time")
        .build();
    counter.add(1, &[KeyValue::new("example_key", "example_value")]);

    loop {
        // Simulate some work
        std::thread::sleep(std::time::Duration::from_secs(1));
        let random_counter: u64 = rand::random::<u64>() % 10;
        counter.add(random_counter, &[KeyValue::new("example_key", "example_value")]);
    }

    // TODO meter.shutdown();

    
}   