use axum::{
    routing::{get, post},
    Router,
};
use tower_http::trace::TraceLayer;
mod models;
// use chrono;
use dotenv::dotenv;
use models::User;
use sqlx::{Pool, Postgres};
use tracing::info;
// use serde::{Deserialize, Serialize};
// use sqlx::Connection;
// use sqlx::Row;
use std::{collections::HashMap, error::Error};
// use tracing_subscriber::layer::SubscriberExt;
mod handlers;
use socketioxide::{extract::SocketRef, SocketIo};

// #[derive(Clone)]
// struct AppState {
//     numbers: Vec<i32>,
//     pool: Pool<Postgres>,
// }

// type AppStateType = Arc<RwLock<AppState>>;

async fn on_connect(socket: SocketRef) {
    info!("socket connected: {}", socket.id)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // initialize tracing

    // let mut cache: HashMap<u32, User> = HashMap::new();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let (socker_layer, io) = SocketIo::new_layer();

    io.ns("/", on_connect);

    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::postgres::PgPool::connect(&db_url).await.unwrap();
    // let state = AppState {
    //     numbers: vec![1, 2, 3],
    //     pool,
    // };
    // sqlx::migrate!("./migrations").run(&pool).await?;

    let app = Router::new()
        .route("/", get(root))
        .route(
            "/:facility_id/sensors/ingest",
            post(handlers::ingest_sensor_data),
        )
        .route("/login", post(handlers::login_handler))
        .route("/user/create", post(handlers::create_user_handler))
        .route("/models/list", post(handlers::list_models))
        .route("/models/:model_id/predict", post(handlers::predict_model))
        .layer(TraceLayer::new_for_http())
        .layer(socker_layer)
        // .with_state(state);
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
