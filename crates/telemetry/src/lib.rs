use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TelemetryError {
    #[error("Failed to initialize tracing: {0}")]
    InitFailed(String),
}

pub fn init_telemetry(_app_name: &str) -> Result<(), TelemetryError> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,api=debug,common=debug,plato_config=debug"));

    let formatting_layer = tracing_subscriber::fmt::layer()
        .pretty()
        .with_target(true);

    // Note: Future OpenTelemetry exporter initialization will go here.
    // e.g., setting up the OTLP exporter and attaching it as a layer.

    Registry::default()
        .with(env_filter)
        .with(formatting_layer)
        .try_init()
        .map_err(|e| TelemetryError::InitFailed(e.to_string()))?;

    Ok(())
}
