use log::{error, warn, info, Level};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    simple_logger::init().unwrap();
    // maximum level of detail.
    log::set_max_level(Level::Info.to_level_filter());
    
    info!(target: "main", "Name: {} Version: {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    warn!(target: "main", "This is a warning message");
    // Emit logs using macros from the log crate.
    // These logs gets piped through OpenTelemetry bridge and gets exported to stdout.
    error!(target: "my-target", "hello from {}. My price is {}", "apple", 2.99);
    loop {
        info!(target: "another-target", "Hello from logs-basic-example");
        sleep(Duration::from_secs(10));
    }
}