use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_tracing() {
    // Determine if we're in production or development
    let is_production = std::env::var("RUST_ENV")
        .unwrap_or_else(|_| "development".to_string())
        .to_lowercase()
        == "production";

    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "info,backend=debug,diesel=info,actix_web=info".into());

    if is_production {
        // Production: JSON structured logging
        let prod_formatting_layer = tracing_subscriber::fmt::layer()
            .with_target(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_file(true)
            .with_line_number(true)
            .pretty()
            .json();

        tracing_subscriber::registry()
            .with(env_filter)
            .with(prod_formatting_layer)
            .init();

        info!("Tracing initialized with structured JSON logging (Production Mode)");
    } else {
        // Development: Beautiful colored logging
        let dev_formatting_layer = tracing_subscriber::fmt::layer()
            .with_target(true)
            .with_thread_ids(false) // Less noise in dev
            .with_thread_names(false)
            .with_file(true)
            .with_line_number(true)
            .with_ansi(true) // Enable colors
            .pretty() // Pretty formatting
            .with_timer(tracing_subscriber::fmt::time::ChronoUtc::rfc_3339());

        tracing_subscriber::registry()
            .with(env_filter)
            .with(dev_formatting_layer)
            .init();

        info!("🎨 Tracing initialized with beautiful colored logging (Development Mode)");
    }
}
