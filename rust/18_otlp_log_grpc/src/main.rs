use log::{Level, error, info, warn};
use opentelemetry::KeyValue;
use opentelemetry_appender_log::OpenTelemetryLogBridge;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{Resource, logs::SdkLoggerProvider};
use opentelemetry_stdout;
// https://docs.rs/opentelemetry-semantic-conventions/latest/opentelemetry_semantic_conventions/
// https://github.com/open-telemetry/opentelemetry-rust/blob/main/opentelemetry-semantic-conventions/src/resource.rs
use opentelemetry_semantic_conventions::resource::{
    K8S_NAMESPACE_NAME, SERVICE_NAME, SERVICE_VERSION,
};

fn init_logger() -> SdkLoggerProvider {
    let endpoint_url = std::env::var("OTLP_LOGGING_BACKEND_URL")
        .unwrap_or_else(|_| "http://localhost:4317".to_string());

    // Create a resource with service information
    // https://docs.rs/opentelemetry_sdk/latest/opentelemetry_sdk/struct.Resource.html
    let resource = Resource::builder_empty()
        .with_attribute(KeyValue::new(SERVICE_NAME, "api-service"))
        .with_attribute(KeyValue::new(SERVICE_VERSION, "0.1.0"))
        .with_attribute(KeyValue::new(K8S_NAMESPACE_NAME, "microservice-simulator"))
        .build();

    // https://opentelemetry.io/docs/languages/rust/exporters/
    // https://docs.rs/opentelemetry-otlp/latest/opentelemetry_otlp/
    // https://github.com/open-telemetry/opentelemetry-rust/blob/main/opentelemetry-otlp/src/logs.rs
    // https://docs.rs/crate/tonic/latest
    // https://github.com/open-telemetry/opentelemetry-rust/blob/main/opentelemetry-otlp/examples/basic-otlp/src/main.rs
    // https://github.com/open-telemetry/opentelemetry-rust
    /*
     with_tonic(): Use the tonic gRPC client for exporting data. Requires grpc-tonic feature to be enabled for opentelemetry-otlp..
     with_endpoint(endpoint_url): Specifies the OTLP endpoint URL where the logs will be sent.

     TODO the exporter is what is connecting to the backend/collector.
    */
    let otlp_exporter = opentelemetry_otlp::LogExporter::builder()
        .with_tonic()
        .with_endpoint(endpoint_url)
        .build()
        .expect("Failed to create log exporter");

    // Create stdout exporter
    let stdout_exporter = opentelemetry_stdout::LogExporter::default();

    let log_provider = SdkLoggerProvider::builder()
        .with_batch_exporter(otlp_exporter)
        .with_simple_exporter(stdout_exporter)
        .with_resource(resource)
        .build();

    // Setup Log Appender for the log crate.
    let otel_log_appender = OpenTelemetryLogBridge::new(&log_provider);
    // TODO what does this do?
    log::set_boxed_logger(Box::new(otel_log_appender)).unwrap();
    // TODO what does this do?
    log::set_max_level(Level::Info.to_level_filter());

    // TODO does it need to be made global? probably not since it is being bridged to the log crate.
    // TODO how to send the logs both to OTLP and stdout?
    
    log_provider
}

#[tokio::main]
async fn main() {
    let log_provider = init_logger();

    info!(target: "my-target", "hello from {}. My price is {}", "apple", 1.99);
    error!(target: "my-target", "Simulated an error occurred: {}", "file not found");
    warn!(target: "my-target", "This is a warning about {}", "low disk space");

    // Force flush logs before exiting - give the batch exporter time to send data
    println!("Flushing logs...");
    if let Err(e) = log_provider.force_flush() {
        eprintln!("Failed to flush logs: {:?}", e);
    }
    
    // Give a bit more time for network I/O to complete
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    println!("Logs should have been sent to OTLP endpoint");
}
