use chrono::serde::ts_seconds;
use chrono::serde::ts_seconds_option;
use chrono::DateTime;
use chrono::Utc;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
pub use validator::Validate;

#[derive(sqlx::Type, Serialize, Deserialize)]
enum EventType {
    #[sqlx(rename = "A")]
    NeighborHelp,
    #[sqlx(rename = "B")]
    Happening,
    #[sqlx(rename = "C")]
    Charity,
}

#[derive(Serialize)]
pub struct Marker {
    latitude: f64,
    longtitude: f64,
    title: String,
    event_type: EventType,
}

#[derive(Deserialize, Serialize, Validate)]
struct Address<'r> {
    street: &'r str,
    #[validate(length(min = 6, max = 6))]
    #[serde(rename = "postalCode")]
    postal_code: &'r str,
    country: &'r str,
    number: u32,
}

#[derive(Serialize, Deserialize)]
pub struct FullMarker<'r> {
    latitude: f64,
    longtitude: f64,
    title: &'r str,
    description: &'r str,
    #[serde(rename = "type")]
    event_type: &'r str,
    #[serde(with = "ts_seconds")]
    #[serde(rename = "addTime")]
    add_time: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    #[serde(rename = "endTime")]
    end_time: Option<DateTime<Utc>>,
    address: Address<'r>,
    #[serde(rename = "contactInfo")]
    contact_info: &'r str,
    #[serde(rename = "userID")]
    user_id: i32,
}

#[derive(Deserialize, Serialize, Validate)]
struct AddressOwned {
    street: String,
    #[validate(length(min = 6, max = 6))]
    #[serde(rename = "postalCode")]
    postal_code: String,
    country: String,
    number: u32,
}
#[derive(Serialize, Deserialize)]
pub struct FullMarkerOwned {
    id: u32,
    latitude: f64,
    longtitude: f64,
    title: String,
    description: String,
    #[serde(rename = "type")]
    r#type: String,
    #[serde(with = "ts_seconds")]
    #[serde(rename = "addTime")]
    add_time: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    #[serde(rename = "endTime")]
    end_time: Option<DateTime<Utc>>,
    address: String,
    #[serde(rename = "contactInfo")]
    contact_info: String,
    #[serde(rename = "userID")]
    user_id: i32,
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

pub async fn show_marker(db: &sqlx::MySqlPool, id: u32) -> anyhow::Result<FullMarkerOwned> {
    let marker = sqlx::query_as!(
        FullMarkerOwned,
        r#"
        SELECT * FROM markers
        Where id = ?
        "#,
        id
    )
    .fetch_one(db)
    .await?;

    Ok(marker)
}
impl<'r> FullMarker<'r> {
    pub async fn add_marker(&self, db: &sqlx::MySqlPool) -> anyhow::Result<bool> {
        let added = sqlx::query!(
            r#"
            INSERT INTO `markers` (`latitude`, `longtitude`, `title`, `description`,
            `type`, `add_time`, `end_time`, `address`, `contact_info`, `user_id`) 
            VALUES (?,?,?,?,?,?,?,?,?,?)"#,
            self.latitude,
            self.longtitude,
            self.title,
            self.description,
            self.event_type.to_string(),
            self.add_time,
            self.end_time,
            serde_json::to_string(&self.address)?,
            self.contact_info,
            self.user_id
        )
        .execute(db)
        .await?;

        Ok(added.rows_affected() > 0)
    }
}
