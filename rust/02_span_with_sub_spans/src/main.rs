use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use opentelemetry::global::{shutdown_tracer_provider};
use opentelemetry::trace::Span;
use opentelemetry::{
    Context,
    global,
    KeyValue,
    trace::{SpanKind, TraceContextExt, TraceError, Tracer},
};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{runtime, trace as sdktrace, Resource};
use rand::Rng;
use std::error::Error;
use std::{convert::Infallible, net::SocketAddr};

/**
  Instantiate a tracer (tokio thread?)
  Define 
  * where to send the information.
  * the name of the service.
  TODO where do you define how many percent to drop?
  TODO maybe get the service name from the Cargo.toml?
 */
fn init_tracer() -> Result<opentelemetry_sdk::trace::Tracer, TraceError> {
    opentelemetry_otlp::new_pipeline()
        .tracing()// This is what sets the output to tracing. TODO does that mean it can only do one thing at a time?
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://localhost:4317"),
        )
        .with_trace_config( // https://docs.rs/opentelemetry-otlp/0.14.0/opentelemetry_otlp/struct.OtlpTracePipeline.html#method.with_trace_config
            sdktrace::config().with_resource(Resource::new(vec![KeyValue::new(
                "service.name",
                "multi-calls-keyvalue", // this is the name that shows up in the jaeger 'service' drop-down list
            )])),
        )
        .install_batch(runtime::Tokio) // Send the data in batches, rather than as they get generated. Reduces transmision waste?
}

/**
 * Handle the http requests.
 * Generate traces of the execution.
 */
async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());

    // Get the tracer started in `ìnit_tracer`?
    let tracer = global::tracer("multi-calls-global-tracer"); // this is the process name

    // start a 'guard' that I use to put all other spans under.
    let _guard = Context::current_with_span(tracer.start("handle_span")).attach();
    
    // from jaeger example code https://github.com/open-telemetry/opentelemetry-rust/blob/main/opentelemetry-otlp/examples/basic-otlp/src/main.rs
    tracer.in_span("main-operation", |cx| {

        let span = cx.span();
        span.set_attribute(KeyValue::new("my-span-attribute", "my-value"));
        span.add_event(
            "Main span event".to_string(),
            vec![KeyValue::new("foo", "1")],
        );
        tracer.in_span("child-operation...", |cx| {
            let span = cx.span();
            span.add_event("Sub span event", vec![KeyValue::new("bar", "1")]);
        });
    });

    // Start a span for handling the http request
    // TODO I think it gets put under the 'handle_span' because of the call to Context::current_with_span() in the beginning of this function
    let mut span = tracer
        .span_builder(format!("{} {}", req.method(), req.uri().path()))
        .with_kind(SpanKind::Server)
        .start(&tracer);

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/rolldice") => {
            let random_number = rand::thread_rng().gen_range(1..7);
            *response.body_mut() = Body::from(random_number.to_string());
            // is this only accessible for stdout open telemetry? span.set_status(Status::Ok);
            //span.add_event(name, attributes)
            span.add_event("Sub span event", vec![KeyValue::new("number", random_number)]);
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
            // is this only accessible for stdout open telemetry? span.set_status(Status::error("Not Found"));
            span.add_event("Sub span event", vec![KeyValue::new("usupported path", "1")]);
            // TODO how can I mark that this is an error path?
        }
    };

    // this was ambiquos span.end();
    opentelemetry::trace::Span::end(&mut span);

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    println!("Starting app");
    let _tracer = init_tracer().expect("Failed to initialize tracer.");

    // Define the hyper server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{addr}/rolldice");
    // Start the server.
    if let Err(e) = server.await {
        eprintln!("server error: {e}");
    }

    shutdown_tracer_provider();

    Ok(())
}
