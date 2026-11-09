use axum::{Extension, http::{HeaderValue, header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}}};
use sqlx::postgres::PgPoolOptions;
use tower_http::{classify::GrpcCode::Ok, cors::CorsLayer};
use tracing_subscriber::filter::LevelFilter;

use config::Config;
use db::DBClient;

mod models;
mod config;
mod dto;
mod error;
mod db;
mod util;
mod middleware;
mod mail;
mod handler;



#[derive(Debug, Clone)]
pub struct AppState {
    pub env: Config,
    pub db_client: DbClient
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(LevelFilter::DEBUG).init();

    dotenvy::dotenv()?;

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
      .allow_methods([METHOD::GET, METHOD::POST, METHOD::PUT]);

    let db_client = DBClient::new(pool);
    let app_state = AppState {
        env: config,
        db_client,
    };

    let app = Router::new().layer(Extension(app_state)).layer(cors.clone());
    println!("server is running on http://localhost:{}", &config.port);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &config.port)).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
