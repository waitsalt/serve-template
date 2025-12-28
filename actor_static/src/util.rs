use tracing::Level;

pub fn log_init() {
    tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .with_max_level(Level::TRACE)
        .init();
}
