#![windows_subsystem = "windows"]
use backends::error;
use log;
use slint_interface;

const DEFAULT_INTERFACE: &str = "TRANSLATOR_INTERFACE";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    unsafe {
        let rust_log = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
        if vec!["trace", "debug", "info", "warn", "error"]
            .contains(&rust_log.to_lowercase().as_str())
        {
            std::env::set_var("RUST_LOG", rust_log);
        } else {
            std::env::set_var("RUST_LOG", "info");
        }
    }

    env_logger::init();
    log::info!(
        "Logging level: {}",
        std::env::var("RUST_LOG").unwrap_or_else(|_| "warn".to_string())
    );
    log::info!(
        "Designated interface: {}",
        std::env::var(DEFAULT_INTERFACE).unwrap_or_else(|_| "slint".to_string())
    );

    // if let Some(err) = backends::initialise().err() {
    //     match err.downcast_ref::<backends::error::Error>() {
    //         // TODO
    //         Some(backends::error::Error::ConfigFileBadFormat(path, s)) => {
    //             log::error!("No backend found");
    //         }
    //         _ => {}
    //     }
    // };

    Ok(slint_interface::run()?)
}
