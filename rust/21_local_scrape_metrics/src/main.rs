use hyper::header::CONTENT_TYPE;
use hyper::{Method, Request, Response};
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use hyper_util::server::conn::auto::Builder;
use http_body_util::Full;
use bytes::Bytes;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use once_cell::sync::Lazy;
use opentelemetry::{
    metrics::{Counter, Histogram, MeterProvider as _, Unit},
    KeyValue,
};
use opentelemetry_sdk::metrics::SdkMeterProvider;
use prometheus::{Encoder, Registry, TextEncoder};
use std::sync::Arc;
use std::time::SystemTime;

static HANDLER_ALL: Lazy<[KeyValue; 1]> = Lazy::new(|| [KeyValue::new("handler", "all")]);

// I think these are the metrics that are provided
struct AppState {
    registry: Registry,
    http_counter: Counter<u64>,
    http_body_gauge: Histogram<u64>,
    http_req_histogram: Histogram<f64>,
}

async fn serve_req(
    req: Request<hyper::body::Incoming>,
    state: Arc<AppState>,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    println!("Receiving request at path {}", req.uri());
    let request_start = SystemTime::now();

    state.http_counter.add(1, HANDLER_ALL.as_ref());

    let response = match (req.method(), req.uri().path()) {
        (&Method::GET, "/metrics") => {
            let mut buffer = vec![];
            let encoder = TextEncoder::new();
            let metric_families = state.registry.gather();
            encoder.encode(&metric_families, &mut buffer).unwrap();
            state
                .http_body_gauge
                .record(buffer.len() as u64, HANDLER_ALL.as_ref());

            Response::builder()
                .status(200)
                .header(CONTENT_TYPE, encoder.format_type())
                .body(Full::new(Bytes::from(buffer)))
                .unwrap()
        }
        (&Method::GET, "/") => Response::builder()
            .status(200)
            .body(Full::new(Bytes::from("Hello World")))
            .unwrap(),
        _ => Response::builder()
            .status(404)
            .body(Full::new(Bytes::from("Missing Page")))
            .unwrap(),
    };

    state.http_req_histogram.record(
        request_start.elapsed().map_or(0.0, |d| d.as_secs_f64()),
        &[],
    );
    Ok(response)
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let registry = Registry::new();
    let exporter = opentelemetry_prometheus::exporter()
        .with_registry(registry.clone())
        .build()?;
    let provider = SdkMeterProvider::builder().with_reader(exporter).build();

    let meter = provider.meter("hyper-example");
    let state = Arc::new(AppState {
        registry,
        http_counter: meter
            .u64_counter("http_requests_total")
            .with_description("Total number of HTTP requests made.")
            .init(),
        http_body_gauge: meter
            .u64_histogram("example.http_response_size")
            .with_unit(Unit::new("By"))
            .with_description("The metrics HTTP response sizes in bytes.")
            .init(),
        http_req_histogram: meter
            .f64_histogram("example.http_request_duration")
            .with_unit(Unit::new("ms"))
            .with_description("The HTTP request latencies in milliseconds.")
            .init(),
    });

    let addr: SocketAddr = ([0, 0, 0, 0], 9030).into();
    let listener = TcpListener::bind(addr).await?;

    println!("Listening on http://{addr}");

    loop {
        let (stream, _) = listener.accept().await?;
        let state = state.clone();
        
        tokio::task::spawn(async move {
            let service = service_fn(move |req| {
                let state = state.clone();
                serve_req(req, state)
            });
            
            if let Err(err) = Builder::new(hyper_util::rt::TokioExecutor::new())
                .serve_connection(TokioIo::new(stream), service)
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }

    Ok(())
}
