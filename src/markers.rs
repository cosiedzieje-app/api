use crate::sqlx::FromRow;
use rocket::serde::json::Json;
use rocket_db_pools::sqlx::Type;
pub use rocket_db_pools::{sqlx, Database};
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
    longtituge: f64,
    title: String,
    event_type: String,
}

pub async fn show_markers(db: &sqlx::MySqlPool) -> anyhow::Result<String> /* anyhow::Result<Vec<Json<Marker>>> */
{
    let markers: Vec<(f64, f64, String, String)> =
        sqlx::query_as("SELECT latitude, longtitude, title, type from Markers")
            .fetch_all(db)
            .await?;

    assert!(markers.len() > 0);
    let s = markers
        .into_iter()
        .take(3)
        .map(|x| {
            serde_json::to_string(&Marker {
                latitude: x.0,
                longtituge: x.1,
                title: x.2,
                event_type: x.3,
            })
            .unwrap_or_default()
        })
        .collect();

    Ok(s)
    /*
    Ok(vec![Json(Marker {
        latitude: 10f32,
        longtituge: 10f32,
        title: "test",
        event_type: EventType::NeighborHelp,
    })])
        */
}
