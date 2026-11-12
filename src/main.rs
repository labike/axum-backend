use std::sync::Arc;

use axum::{http::{HeaderValue, header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}, Method}};
use sqlx::postgres::PgPoolOptions;
use tower_http::{cors::CorsLayer};
use tracing_subscriber::filter::LevelFilter;

use config::Config;
use db::DBClient;

use crate::routes::create_router;

mod models;
mod config;
mod dto;
mod error;
mod db;
mod util;
mod middleware;
mod mail;
mod handler;
mod routes;


#[derive(Debug, Clone)]
pub struct AppState {
    pub env: Config,
    pub db_client: DBClient,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(LevelFilter::DEBUG).init();

    dotenvy::dotenv();

    let config = Config::init();

    let pool = match PgPoolOptions::new()
      .max_connections(10)
      .connect(&config.database_url)
      .await
    {
        Ok(pool) => {
            println!("connection the database successful!");
            pool
        }
        Err(err) => {
            println!("fail to connection the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let cors = CorsLayer::new()
      .allow_origin("http://localhost:3333".parse::<HeaderValue>().unwrap())
      .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
      .allow_credentials(true)
      .allow_methods([Method::GET, Method::POST, Method::PUT]);

    let db_client = DBClient::new(pool);
    let app_state = AppState {
        env: config.clone(),
        db_client,
    };

    // let app = Router::new().layer(Extension(app_state)).layer(cors.clone());
    let app = create_router(Arc::new(app_state.clone())).layer(cors.clone());
    println!("server is running on http://localhost:{}", &config.port);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &config.port)).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
