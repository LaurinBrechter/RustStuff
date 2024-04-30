use crate::models::{self, UserCreateInfo};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use polars::prelude::*;
use pwhash::bcrypt;
use serde::{Deserialize, Serialize};
use sqlx::{query, Pool, Postgres};

#[derive(Serialize)]
pub struct SensorData {
    id: u64,
    success: bool,
    test: String,
    facility_id: u32,
}

#[derive(Deserialize)]
pub struct QueryParams {
    test: String,
}

#[derive(Deserialize)]
pub struct SensorIngestReq {
    sensor_id: i32,
    timestamp: i32,
    values: Vec<f64>,
    // y: f64,
}

pub async fn ingest_sensor_data(
    Path(facility_id): Path<i64>,
    Query(params): Query<QueryParams>,
    State(pool): State<Pool<Postgres>>,
    // State(cache): State<AppStateType>,
    Json(body): Json<SensorIngestReq>,
) -> StatusCode {
    println!("params: {}", params.test);

    // let mut cache_write = cache.write().unwrap();

    // cache_write.numbers.push(12);

    let insert_result = query!(
        "
        INSERT INTO rust_test_db.sensor_readings 
        (sensor_id, timestamp, value, facility_id) 
        VALUES ($1, $2, $3, $4)",
        body.sensor_id,
        body.timestamp,
        body.values[0],
        facility_id
    )
    .execute(&pool)
    .await;

    match insert_result {
        Ok(_) => return StatusCode::CREATED,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn login_handler(
    // State(state): State<AppState>,
    Json(login_info): Json<models::LoginInfo>,
) -> Result<Json<models::LoginResponse>, StatusCode> {
    // let x = state.read().expect("fuuuuck");

    if login_info.username == "admin" && login_info.password == "password" {
        Ok(Json(models::LoginResponse {
            success: true,
            token: "1234".to_string(),
            // state: x.numbers.clone(),
        }))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

pub async fn create_user_handler(
    // Json(body): Json<models::UserCreateInfo>,
    State(pool): State<sqlx::PgPool>,
    Json(body): Json<UserCreateInfo>,
) -> StatusCode {
    let hash_ = bcrypt::hash(body.password).expect("hash");

    let insert_result = query!(
        "
        INSERT INTO rust_test_db.users 
        (username, email, password) 
        VALUES ($1, $2, $3)",
        body.username,
        body.email,
        hash_,
    )
    .execute(&pool)
    .await;

    match insert_result {
        Ok(_) => return StatusCode::CREATED,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn list_models() {}

pub async fn predict_model(Path(model_id): Path<i64>) {
    let query = LazyCsvReader::new("/data/iris.csv")
        .has_header(true)
        .finish()
        .unwrap()
        .group_by(vec![col("species")])
        .agg([col("*").sum()]);
    let df = query.collect();
}
