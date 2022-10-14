use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
enum EventType {
    NeighborHelp,
    Happening,
    Charity,
}
impl<'r> EventType {
    fn as_str(&self) -> &'r str {
        match self {
            EventType::NeighborHelp => "a",
            EventType::Happening => "b",
            EventType::Charity => "c",
        }
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Marker {
    latitude: f64,
    longtitude: f64,
    title: String,
    event_type: String,
}

pub async fn show_markers(db: &sqlx::MySqlPool) -> anyhow::Result<Vec<Marker>> {
    let markers = sqlx::query_as!(Marker, r#"
    SELECT latitude, longtitude, title, type as event_type
    FROM markers
    "#)
        .fetch_all(db)
        .await?;

    Ok(markers)
}