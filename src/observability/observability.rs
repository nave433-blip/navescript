use tracing::{info, warn, error, debug};
use tracing_subscriber;

pub struct ObservabilityHost;

impl ObservabilityHost {
    pub fn new() -> Self {
        tracing_subscriber::fmt::init();
        ObservabilityHost {}
    }

    pub fn log(&self, level: &str, message: &str) {
        match level.to_lowercase().as_str() {
            "debug" => debug!("{}", message),
            "warn" => warn!("{}", message),
            "error" => error!("{}", message),
            _ => info!("{}", message),
        }
    }
}
