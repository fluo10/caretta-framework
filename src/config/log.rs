/// A Config about logging
#[derive(Clone, Debug)]
pub struct LogConfig {
    pub level: tracing::Level,
}

impl LogConfig {
    pub fn init_tracing_subscriber(&self, verbose: bool) {
        tracing_subscriber::fmt().with_max_level(self.level).init();
    }
}
