use sqlx::{Connection, PgConnection};
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read file");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("failed to connect to Postgres.");
    println!("{}", configuration.application_port);
    let address = format!("127.0.0.1:{}", configuration.application_port);
    println!("{}", address);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
