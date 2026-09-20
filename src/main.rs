use std::sync::Arc;

use surrealdb::{
    Surreal,
    engine::remote::ws::{Client, Wss},
    opt::auth::Database,
};
use tracing::info;
use tracing_subscriber::EnvFilter;

mod db;
mod server;

#[derive(Clone)]
struct AppState {
    db: Arc<Surreal<Client>>,
}

fn get_env(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| panic!("Missing environment variable: {name}"))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::from("INFO")),
        )
        .init();

    let surreal_uri = get_env("SURREAL_URI");
    let surreal_user = get_env("SURREAL_USER");
    let surreal_pass = get_env("SURREAL_PASS");
    let addr = get_env("ADDR");
    let port = get_env("PORT");

    let db = Surreal::new::<Wss>(surreal_uri).await?;

    db.signin(Database {
        namespace: "main".into(),
        database: "main".into(),
        username: surreal_user,
        password: surreal_pass,
    })
    .await?;

    db.use_ns("main").use_db("main").await?;

    info!("Connected to SurrealDB successfully");

    let app = server::router(AppState { db: Arc::new(db) });

    let listener = tokio::net::TcpListener::bind(format!("{addr}:{port}")).await?;
    info!("Listening on {addr}");
    axum::serve(listener, app).await?;

    Ok(())
}
