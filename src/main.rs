use std::net::SocketAddr;
use std::sync::Arc;

mod error;
mod handlers;
mod models;
mod repository;
mod routes;
mod service;

use crate::repository::InMemoryPersonRepository;
use crate::service::PersonService;

fn port() -> u16 {
    std::env::var("PORT")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(8080)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "person_service=debug,tower_http=debug,info".into()),
        )
        .init();

    let repository = Arc::new(InMemoryPersonRepository::new());
    let app = routes::router(PersonService::new(repository));

    let addr = SocketAddr::from(([0, 0, 0, 0], port()));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("person-service listening on {addr}");
    axum::serve(listener, app).await?;

    Ok(())
}
