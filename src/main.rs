use std::net::SocketAddr;
use std::sync::Arc;

mod config;
mod error;
mod handlers;
mod models;
mod postgres;
mod repository;
mod routes;
mod service;

use crate::config::Config;
use crate::postgres::PgPersonRepository;
use crate::service::PersonService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "person_service=debug,tower_http=debug,info".into()),
        )
        .init();

    let config = Config::from_env()?;

    let repository =
        PgPersonRepository::connect(&config.database_url, config.max_connections).await?;
    tracing::info!("connected to the database");

    let app = routes::router(PersonService::new(Arc::new(repository)));

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("person-service listening on {addr}");
    axum::serve(listener, app).await?;

    Ok(())
}
