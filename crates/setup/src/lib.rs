pub mod http;

/// Sets up tracing.
pub fn tracing() {
    tracing_subscriber::fmt::init();
}
