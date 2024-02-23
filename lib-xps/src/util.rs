//! Internal Utility functions for use in crate
use std::sync::Once;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};

static INIT: Once = Once::new();

#[ctor::ctor]
fn __init_test_logging() {
    INIT.call_once(|| {
        let fmt = fmt::layer().compact();
        Registry::default().with(env()).with(fmt).init()
    })
}

/// Try to get the logging environment from the `RUST_LOG` environment variable.
/// If it is not set, use the default of `info`.
pub fn env() -> EnvFilter {
    EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"))
}
