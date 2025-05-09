use std::error::Error;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::Serialize;
use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let database_url = dotenv::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();
    let app = Router::new()
        .route("/", get(|| async { "root" }))
        .route("/aircraft_ids", get(handle_aircraft_ids))
        .route("/track/{aircraft_id}", get(handle_get_track))
        .with_state(pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

#[derive(Serialize)]
struct AircraftIdsRes {
    aircraft_ids: Vec<String>,
}

async fn handle_aircraft_ids(
    State(pool): State<PgPool>,
) -> Result<Json<AircraftIdsRes>, (StatusCode, String)> {
    let ids = get_aircraft_ids(&pool).await.unwrap();
    Ok(Json(AircraftIdsRes { aircraft_ids: ids }))
}

async fn get_aircraft_ids(pool: &Pool<Postgres>) -> Result<Vec<String>, ()> {
    let res = sqlx::query!(
        r#"
        SELECT DISTINCT aircraft_id FROM tracks
        "#,
    )
    .fetch(pool);
    res.map(|res| match res {
        Ok(row) => Ok(row.aircraft_id.unwrap_or("".to_string())),
        Err(_) => Err(()),
    })
    .collect()
    .await
}

type AircraftPosition = (f64, f64);

#[derive(Serialize)]
struct TrackRes {
    positions: Vec<AircraftPosition>,
}

async fn handle_get_track(
    Path(aircraft_id): Path<String>,
    State(pool): State<PgPool>,
) -> Result<Json<TrackRes>, (StatusCode, String)> {
    let positions = get_track(&pool, &aircraft_id).await.unwrap();
    Ok(Json(TrackRes { positions }))
}

async fn get_track(pool: &Pool<Postgres>, aircraft_id: &str) -> Result<Vec<AircraftPosition>, ()> {
    let res = sqlx::query!(
        r#"
        SELECT
            ST_X(position::geometry), ST_Y(position::geometry),
            ST_Z(position::geometry), ST_M(position::geometry)
        FROM tracks WHERE aircraft_id = $1
        "#,
        aircraft_id
    )
    .fetch(pool);
    res.map(|res| match res {
        Ok(row) => {
            if let (Some(x), Some(y)) = (row.st_x, row.st_y) {
                Ok((x, y))
            } else {
                Err(())
            }
        }
        Err(_) => Err(()),
    })
    .collect()
    .await
}
