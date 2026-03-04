use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Initializes the global tracing subscriber.
/// In production, this can be extended to push OTLP spans to Jaeger/Datadog.
pub fn init_tracing() {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,syntropy=debug"));

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .init();

    tracing::info!("Syntropy observability initialized.");
}
