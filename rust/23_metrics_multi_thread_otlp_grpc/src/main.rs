use axum::{Router, extract::Query, http::StatusCode, response::Json, routing::get};
use opentelemetry::{KeyValue, global};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{Resource, metrics::SdkMeterProvider};
use rand::RngExt;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Deserialize)]
struct DiceQuery {
    count: Option<i32>,
}

#[derive(Serialize)]
struct RollResult {
    dice: String,
    count: i32,
    rolls: Vec<u32>,
    total: u32,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
}

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
        .with_attribute(KeyValue::new("service.namespace", "23_metrics_multi_thread_otlp_grpc"))
        .build();

    let matrics_provider = SdkMeterProvider::builder()
        .with_periodic_exporter(otlp_exporter)
        .with_resource(resource)
        .build();

    global::set_meter_provider(matrics_provider);
    Ok(())
}

async fn roll_d6(
    Query(params): Query<DiceQuery>,
) -> Result<Json<RollResult>, (StatusCode, Json<ErrorResponse>)> {
    let meter = global::meter("MetricMultiThreadSpike");
    let counter = meter
        .u64_counter("roll_d6_calls")
        .with_description("Number of times the roll_d6 endpoint was called")
        .build();
    counter.add(1, &[]);
    let total_connections = meter
        .u64_counter("total_connections")
        .build();
    total_connections.add(1, &[]);
    let active = meter
        .i64_up_down_counter("active_requests")
        .with_description("Number of requests currently being executed")
        .build();
    active.add(1, &[KeyValue::new("handler", "roll_d6")]);

    let count = params.count.unwrap_or(1);

    if count <= 0 {
        active.add(-1, &[KeyValue::new("handler", "roll_d6")]);
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: format!("count must be between 1 and 18, got {count}"),
            }),
        ));
    }
    if count > 18 {
        active.add(-1, &[KeyValue::new("handler", "roll_d6")]);
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: format!("count must be between 1 and 18, got {count}"),
            }),
        ));
    }

    let mut rng = rand::rng();
    let rolls: Vec<u32> = (0..count).map(|_| rng.random_range(1..=6)).collect();
    let total: u32 = rolls.iter().sum();

    active.add(-1, &[KeyValue::new("handler", "roll_d6")]);
    Ok(Json(RollResult {
        dice: "d6".to_string(),
        count,
        rolls,
        total,
    }))
}

async fn roll_d8(
    Query(params): Query<DiceQuery>,
) -> Result<Json<RollResult>, (StatusCode, Json<ErrorResponse>)> {
    let meter = global::meter("MetricMultiThreadSpike");
    let counter = meter
        .u64_counter("roll_d8_calls")
        .with_description("Number of times the roll_d8 endpoint was called")
        .build();
    counter.add(1, &[]);
    let total_connections = meter
        .u64_counter("total_connections")
        .build();
    total_connections.add(1, &[]);
    let active = meter
        .i64_up_down_counter("active_requests")
        .with_description("Number of requests currently being executed")
        .build();
    active.add(1, &[KeyValue::new("handler", "roll_d8")]);

    let count = params.count.unwrap_or(1);

    if count < 1 || count > 10 {
        active.add(-1, &[KeyValue::new("handler", "roll_d8")]);
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: format!("count must be between 1 and 10, got {count}"),
            }),
        ));
    }

    let mut rng = rand::rng();
    let rolls: Vec<u32> = (0..count).map(|_| rng.random_range(1..=8)).collect();
    let total: u32 = rolls.iter().sum();

    active.add(-1, &[KeyValue::new("handler", "roll_d8")]);
    Ok(Json(RollResult {
        dice: "d8".to_string(),
        count,
        rolls,
        total,
    }))
}

async fn health() -> Json<HealthResponse> {
    let meter = global::meter("MetricMultiThreadSpike");
    let active = meter
        .i64_up_down_counter("active_requests")
        .with_description("Number of requests currently being executed")
        .build();
    active.add(1, &[KeyValue::new("handler", "health")]);
    let result = Json(HealthResponse {
        status: "ok".to_string(),
    });
    active.add(-1, &[KeyValue::new("handler", "health")]);
    result
}

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
    let meter = global::meter("MetricMultiThreadSpike");
    // Inits the counter and set description, once and for all.
    let _counter = meter
        .u64_counter("total_connections")
        .with_description("Total connections over time")
        .build();

    let app = Router::new()
        .route("/d6", get(roll_d6))
        .route("/d8", get(roll_d8))
        .route("/health", get(health));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    //         counter.add(random_counter, &[KeyValue::new("example_key", "example_value")]);
}
