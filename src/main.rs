use sqlx::PgPool;
use std::{net::TcpListener};
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use tracing_log::LogTracer;

pub fn get_subscriber(
    name: String,
    env_filter: String
) -> impl Subscriber + Send + Sync {
      // Sets a fallback filter of INFO if no env variable set for a span filter
      let env_filter = EnvFilter::try_from_default_env()
      .unwrap_or_else(|_| EnvFilter::new(env_filter));
  // output formatted spans to stdout
  let formatting_layer = BunyanFormattingLayer::new(
      name,
      std::io::stdout
  );

  Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
   // Redirect all 'log''s events to our subscriber
   LogTracer::init().expect("Failed to set logger");

    // `set_global_default` can be used by apps to specify what subscriber should be used to process spans
    set_global_default(subscriber).expect("Failed to set subscriber");
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into());
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read file");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
