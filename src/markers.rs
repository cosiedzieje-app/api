use rocket::{serde::json::Json, response::stream::Event};
use serde::{Deserialize, Serialize};

#[derive(sqlx::Type, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
enum EventType {
    #[sqlx(rename = "A")]
    NeighborHelp,
    #[sqlx(rename = "B")]
    Happening,
    #[sqlx(rename = "C")]
    Charity,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Marker {
    latitude: f64,
    longtitude: f64,
    title: String,
    event_type: EventType,
}

pub async fn show_markers(db: &sqlx::MySqlPool) -> anyhow::Result<Vec<Marker>> {
    let markers = sqlx::query_as!(Marker, r#"
    SELECT latitude, longtitude, title, type as `event_type: EventType`
    FROM markers
    "#)
        .fetch_all(db)
        .await?;

    Ok(markers)
}