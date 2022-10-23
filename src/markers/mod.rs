use chrono::serde::{ts_seconds, ts_seconds_option};
use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
pub use validator::Validate;

use crate::users::login::AddressOwned;
use crate::users::register::Address;

#[derive(sqlx::Type, Serialize, Deserialize)]
enum EventType {
    #[sqlx(rename = "A")]
    NeighborHelp,
    #[sqlx(rename = "B")]
    Happening,
    #[sqlx(rename = "C")]
    Charity,
    #[sqlx(rename = "D")]
    MassEvent,
}

#[derive(Serialize, Deserialize /* , sqlx::Type */)]
#[serde(tag = "type", content = "val")]
enum ContactMethod {
    Email(String),
    PhoneNumber(String),
}

#[derive(Serialize, Deserialize)]
pub struct ContactInfo {
    name: String,
    surname: String,
    address: AddressOwned,
    method: ContactMethod,
}

#[derive(Serialize, Deserialize)]
pub struct FullMarker<'r> {
    latitude: f64,
    longitude: f64,
    title: &'r str,
    description: &'r str,
    #[serde(rename = "type")]
    r#type: EventType,
    #[serde(with = "ts_seconds")]
    #[serde(rename = "addTime")]
    #[serde(default)]
    add_time: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    #[serde(rename = "startTime")]
    #[serde(default)]
    start_time: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    #[serde(rename = "endTime")]
    #[serde(default)]
    end_time: Option<DateTime<Utc>>,
    address: Address<'r>,
    #[serde(rename = "contactInfo")]
    contact_info: ContactInfo,
}

#[derive(Serialize, Deserialize)]
pub struct FullMarkerOwned {
    id: u32,
    latitude: f64,
    longitude: f64,
    title: String,
    description: String,
    #[serde(rename = "type")]
    r#type: EventType,
    #[serde(with = "ts_seconds")]
    #[serde(rename = "addTime")]
    #[serde(default)]
    add_time: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    #[serde(rename = "startTime")]
    #[serde(default)]
    start_time: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    #[serde(rename = "endTime")]
    #[serde(default)]
    end_time: Option<DateTime<Utc>>,
    address: sqlx::types::Json<AddressOwned>,
    #[serde(rename = "contactInfo")]
    contact_info: sqlx::types::Json<ContactInfo>,
    #[serde(rename = "userID")]
    user_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct FullMarkerOwnedWithDist {
    id: u32,
    latitude: f64,
    longitude: f64,
    title: String,
    description: String,
    #[serde(rename = "type")]
    r#type: EventType,
    #[serde(with = "ts_seconds")]
    #[serde(rename = "addTime")]
    #[serde(default)]
    add_time: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    #[serde(rename = "startTime")]
    #[serde(default)]
    start_time: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    #[serde(rename = "endTime")]
    #[serde(default)]
    end_time: Option<DateTime<Utc>>,
    address: sqlx::types::Json<AddressOwned>,
    #[serde(rename = "contactInfo")]
    contact_info: sqlx::types::Json<ContactInfo>,
    #[serde(rename = "distanceInKm")]
    distance_in_km: Option<f64>,
    #[serde(rename = "userID")]
    user_id: i32,
}

pub async fn delete_marker(
    db: &sqlx::MySqlPool,
    user_id: u32,
    marker_id: u32,
) -> anyhow::Result<FullMarkerOwned> {
    let mut tx = db.begin().await?;

    let marker = sqlx::query_as!(
        FullMarkerOwned,
        r#"
        SELECT id, latitude, longitude, title, description, type as `type: EventType`, add_time, start_time, end_time,
        address as `address: sqlx::types::Json<AddressOwned>`, contact_info as 'contact_info: sqlx::types::Json<ContactInfo>', user_id
        FROM markers
        Where id = ? AND user_id = ?
        "#,
        marker_id,user_id
    )
    .fetch_one(&mut tx)
    .await?;

    sqlx::query!(
        r#"
            DELETE FROM markers WHERE id = ? AND user_id = ?   
            "#,
        marker_id,
        user_id
    )
    .execute(&mut tx)
    .await?;

    tx.commit().await?;

    Ok(marker)
}

pub async fn show_markers(db: &sqlx::MySqlPool) -> anyhow::Result<Vec<FullMarkerOwned>> {
    let markers = sqlx::query_as!(
        FullMarkerOwned,
        r#"
        SELECT id, latitude, longitude, title, description, type as `type: EventType`, add_time,start_time, end_time,
        address as `address: sqlx::types::Json<AddressOwned>`, contact_info as 'contact_info: sqlx::types::Json<ContactInfo>', user_id
        FROM markers
        "#
    )
    .fetch_all(db)
    .await?;

    Ok(markers)
}

pub async fn show_markers_by_city(
    db: &sqlx::MySqlPool,
    city: &str,
) -> anyhow::Result<Vec<FullMarkerOwned>> {
    let markers = sqlx::query_as!(
        FullMarkerOwned,
        r#"
        SELECT id, latitude, longitude, title, description, type as `type: EventType`, add_time,start_time, end_time,
        address as `address: sqlx::types::Json<AddressOwned>`, contact_info as 'contact_info: sqlx::types::Json<ContactInfo>', user_id
        FROM markers WHERE JSON_EXTRACT(address,"$.city") = ?
        "#,
        city
    )
    .fetch_all(db)
    .await?;

    Ok(markers)
}

pub async fn show_markers_by_dist(
    db: &sqlx::MySqlPool,
    x: f64,
    y: f64,
    dist: u32,
) -> anyhow::Result<Vec<FullMarkerOwnedWithDist>> {
    // SELECT id, latitude, longitude, title, type as `event_type: EventType`,user_id
    // Thanks for the formula: http://www.plumislandmedia.net/mysql/haversine-mysql-nearest-loc/
    let markers = sqlx::query_as!(
        FullMarkerOwnedWithDist,
        r#"
        SELECT z.id, z.latitude, z.longitude, z.title, z.description, z.type as `type: EventType`, z.add_time,start_time, z.end_time,
        z.address as `address: sqlx::types::Json<AddressOwned>`, z.contact_info as 'contact_info: sqlx::types::Json<ContactInfo>', z.user_id,
        p.distance_unit
                * DEGREES(ACOS(LEAST(1.0, COS(RADIANS(p.latpoint))
                * COS(RADIANS(z.latitude))
                * COS(RADIANS(p.longpoint) - RADIANS(z.longitude))
                + SIN(RADIANS(p.latpoint))
                * SIN(RADIANS(z.latitude))))) AS distance_in_km
        FROM markers AS z
        JOIN (   /* these are the query parameters */
            SELECT  ?  AS latpoint,      ? AS longpoint,
                    ? AS radius,      111.045 AS distance_unit
        ) AS p ON 1=1
        WHERE z.latitude
        BETWEEN p.latpoint  - (p.radius / p.distance_unit)
            AND p.latpoint  + (p.radius / p.distance_unit)
        AND z.longitude
        BETWEEN p.longpoint - (p.radius / (p.distance_unit * COS(RADIANS(p.latpoint))))
            AND p.longpoint + (p.radius / (p.distance_unit * COS(RADIANS(p.latpoint))))
        ORDER BY distance_in_km
        LIMIT 15;
        "#,
        x,
        y,
        dist
    )
    .fetch_all(db)
    .await?;

    Ok(markers)
}

pub async fn show_user_markers(
    db: &sqlx::MySqlPool,
    user_id: u32,
) -> anyhow::Result<Vec<FullMarkerOwned>> {
    let markers = sqlx::query_as!(
        FullMarkerOwned,
        r#"
        SELECT id, latitude, longitude, title, description, type as `type: EventType`, add_time,start_time, end_time,
        address as `address: sqlx::types::Json<AddressOwned>`, contact_info as 'contact_info: sqlx::types::Json<ContactInfo>', user_id
        FROM markers WHERE user_id = ?
        "#,
        user_id
    )
    .fetch_all(db)
    .await?;

    Ok(markers)
}

/* pub async fn show_marker(db: &sqlx::MySqlPool, id: u32) -> anyhow::Result<FullMarkerOwned> {
    let marker = sqlx::query_as!(
        FullMarkerOwned,
        r#"
        SELECT id, latitude, longitude, title, description, type as `type: EventType`, add_time,start_time, end_time,
        address as `address: sqlx::types::Json<AddressOwned>`, contact_info as 'contact_info: sqlx::types::Json<ContactInfo>', user_id
        FROM markers Where id = ?
        "#,
        id
    )
    .fetch_one(db)
    .await?;

    Ok(marker)
} */
impl<'r> FullMarker<'r> {
    pub async fn add_marker(&self, db: &sqlx::MySqlPool, user_id: u32) -> anyhow::Result<bool> {
        let added = sqlx::query!(
            r#"
            INSERT INTO `markers` (`latitude`, `longitude`, `title`, `description`,
            `type`, `add_time`, `start_time`, `end_time`, `address`, `contact_info`, `user_id`) 
            VALUES (?,?,?,?,?,?,?,?,?,?,?)"#,
            self.latitude,
            self.longitude,
            self.title,
            self.description,
            self.r#type,
            chrono::offset::Utc::now(),
            self.start_time,
            self.end_time,
            serde_json::to_string(&self.address)?,
            serde_json::to_string(&self.contact_info)?,
            user_id
        )
        .execute(db)
        .await?;

        Ok(added.rows_affected() > 0)
    }
}
