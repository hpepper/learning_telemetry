use log::{ error, info, LevelFilter, warn };
use std::thread::sleep;
use std::time::Duration;

macro_rules! format_file_and_line {
    ($data:expr) => (
        format!(r#"{{ "filename": "{}", "line": {}, "data": "{}" }}"#, file!(), line!(), $data)
    )
}

fn main() {
    // Initialize logger with JSON format
    json_log::init_with_level(LevelFilter::Info).unwrap();

    let tada_im_running = format!("Name: {} Version: {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    info!("{}", format_file_and_line!(tada_im_running));
    /*
    // Your application code here
    let filename = file!(); // Get the current filename
    let line_number = line!(); // Get the current line number

    // Create a JSON object as a string and log it
    let log_message = format!(
        r#"{{ "message": "Starting application", "filename": "{}", "line": {} }}"#,
        filename,
        line_number
    );
    log::info!("{}", log_message);
    error!(target: "my-target", "hello from {}. My price is {}", "apple", 2.99);
     */
    warn!("{}", format_file_and_line!("Hello, world!"));
    loop {
        info!("{}", format_file_and_line!("time info"));
        sleep(Duration::from_secs(10));
    }

}

// https://stackoverflow.com/questions/63554693/fluent-bit-filter-to-convert-unix-epoch-timestamp-to-human-readable-time-format
/*
[PARSER]
        Name   json
        Format json
        Time_Key timeMillis
        Time_Format %s
        Time_Keep On
 */

 /*
#[macro_use]
extern crate log;

use log::{LevelFilter};

// Define a macro to simplify logging with filename and line number
macro_rules! log_with_filename_and_line {
    ($lvl:expr, $msg:expr) => {{
        let filename = file!(); // Get the current filename
        let line_number = line!(); // Get the current line number
        
        // Create a JSON object as a string and log it
        let log_message = format!(
            r#"{{ "message": "{}", "filename": "{}", "line": {} }}"#,
            $msg, filename, line_number
        );

        // Log the JSON message at the specified log level
        log::log!($lvl, "{}", log_message);
    }};
}

fn main() {
    // Initialize logger with JSON format
    json_log::init_with_level(LevelFilter::Info).unwrap();

    // Your application code here
    log_with_filename_and_line!(log::Level::Info, "Starting application");
}
  */


/*
use log::{error, info, LevelFilter, warn};

fn main() {
    // Initialize logger
    json_log::init_with_level(LevelFilter::Info).unwrap(); // use the `INFO` level.

    // Your application code here
    info!("Starting application");

}
 */
