pub use tracing::level_filters::LevelFilter;
pub use tracing::{debug, debug_span, error, error_span, info, info_span, trace, trace_span, warn, warn_span};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

pub fn init_tracer(app_name: &str) {
  let app_name = app_name.to_string();
  let formatting_layer = BunyanFormattingLayer::new(app_name, std::io::stdout);
  let filter = EnvFilter::builder()
    .with_default_directive(LevelFilter::INFO.into())
    .from_env_lossy()
    .add_directive("hyper=off".parse().unwrap());

  let subscriber = Registry::default().with(JsonStorageLayer).with(formatting_layer).with(filter);

  tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
}
