use std::sync::Arc;
use tracing_log::LogTracer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

#[track_caller]
#[inline(always)]
pub fn init_logging(enable_colors: bool) {
    if enable_colors {
        color_eyre::install().expect("failed to install color_eyre");
    }

    LogTracer::builder()
        .ignore_crate("rustls")
        .with_max_level(log::LevelFilter::Debug)
        .init()
        .expect("failed to initialize LogTracer");

    let env_filter =
        EnvFilter::try_from_env("LOG_LEVEL").unwrap_or_else(|_| EnvFilter::new("info"));

    let subscriber = Registry::default()
        .with(tracing_subscriber::fmt::layer().pretty())
        .with(tracing_error::ErrorLayer::default())
        .with(env_filter);
    let subscriber = Arc::new(subscriber);

    tracing::subscriber::set_global_default(subscriber).expect("failed to set tracing subscriber");

    std::panic::set_hook(Box::new(|err| {
        let payload = err.payload();

        let location = err
            .location()
            .copied()
            .map(|loc| format!("{}", loc))
            .unwrap_or_default();

        if let Some(payload) = payload.downcast_ref::<String>() {
            tracing::error!(location, "panic:\n{payload}");
        } else if let Some(&payload) = payload.downcast_ref::<&'static str>() {
            tracing::error!(location, "panic:\n{payload}");
        } else {
            tracing::error!(location, "panic with unknown payload");
        }
    }));
}
