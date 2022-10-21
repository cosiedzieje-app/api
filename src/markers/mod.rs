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

#[derive(Serialize)]
pub struct Marker {
    id: u32,
    latitude: f64,
    longtitude: f64,
    title: String,
    #[serde(rename = "type")]
    event_type: EventType,
    #[serde(rename = "userID")]
    user_id: i32,
}

#[derive(Serialize)]
pub struct MarkerWithDist {
    id: u32,
    latitude: f64,
    longtitude: f64,
    title: String,
    #[serde(rename = "type")]
    event_type: EventType,
    #[serde(rename = "userID")]
    user_id: i32,
    distance_in_km: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct FullMarker<'r> {
    latitude: f64,
    longtitude: f64,
    title: &'r str,
    description: &'r str,
    #[serde(rename = "type")]
    event_type: EventType,
    #[serde(with = "ts_seconds")]
    #[serde(rename = "addTime")]
    #[serde(default)]
    add_time: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    #[serde(rename = "endTime")]
    end_time: Option<DateTime<Utc>>,
    address: Address<'r>,
    #[serde(rename = "contactInfo")]
    contact_info: ContactInfo,
}

#[derive(Serialize, Deserialize)]
pub struct FullMarkerOwned {
    id: u32,
    latitude: f64,
    longtitude: f64,
    title: String,
    description: String,
    #[serde(rename = "type")]
    r#type: EventType,
    #[serde(with = "ts_seconds")]
    #[serde(rename = "addTime")]
    #[serde(default)]
    add_time: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    #[serde(rename = "endTime")]
    end_time: Option<DateTime<Utc>>,
    address: sqlx::types::Json<AddressOwned>,
    #[serde(rename = "contactInfo")]
    contact_info: sqlx::types::Json<ContactInfo>,
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
        SELECT id, latitude, longtitude, title, description, type as `type: EventType`, add_time, end_time,
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
pub async fn show_markers(db: &sqlx::MySqlPool) -> anyhow::Result<Vec<Marker>> {
    let markers = sqlx::query_as!(
        Marker,
        r#"
    SELECT id, latitude, longtitude, title, type as `event_type: EventType`,user_id
    FROM markers
    "#
    )
    .fetch_all(db)
    .await?;

    Ok(markers)
}

pub async fn show_markers_by_city(db: &sqlx::MySqlPool, city: &str) -> anyhow::Result<Vec<Marker>> {
    let markers = sqlx::query_as!(
        Marker,
        r#"
    SELECT id, latitude, longtitude, title, type as `event_type: EventType`,user_id
    FROM markers
    WHERE JSON_EXTRACT(address,"$.city") = ?
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
) -> anyhow::Result<Vec<MarkerWithDist>> {
    // SELECT id, latitude, longtitude, title, type as `event_type: EventType`,user_id
    // Thanks for the formula: http://www.plumislandmedia.net/mysql/haversine-mysql-nearest-loc/
    let markers = sqlx::query_as!(
        MarkerWithDist,
        r#"
        SELECT 
        z.id, z.latitude, z.longtitude, z.title, z.type as `event_type: EventType`,user_id,
        p.distance_unit
                * DEGREES(ACOS(LEAST(1.0, COS(RADIANS(p.latpoint))
                * COS(RADIANS(z.latitude))
                * COS(RADIANS(p.longpoint) - RADIANS(z.longtitude))
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
        AND z.longtitude
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
        SELECT id, latitude, longtitude, title, description, type as `type: EventType`, add_time, end_time,
        address as `address: sqlx::types::Json<AddressOwned>`, contact_info as 'contact_info: sqlx::types::Json<ContactInfo>', user_id
        FROM markers WHERE user_id = ?
        "#,
        user_id
    )
    .fetch_all(db)
    .await?;

    Ok(markers)
}

pub async fn show_marker(db: &sqlx::MySqlPool, id: u32) -> anyhow::Result<FullMarkerOwned> {
    let marker = sqlx::query_as!(
        FullMarkerOwned,
        r#"
        SELECT id, latitude, longtitude, title, description, type as `type: EventType`, add_time, end_time,
        address as `address: sqlx::types::Json<AddressOwned>`, contact_info as 'contact_info: sqlx::types::Json<ContactInfo>', user_id
        FROM markers Where id = ?
        "#,
        id
    )
    .fetch_one(db)
    .await?;

    Ok(marker)
}
impl<'r> FullMarker<'r> {
    pub async fn add_marker(&self, db: &sqlx::MySqlPool, user_id: u32) -> anyhow::Result<bool> {
        let added = sqlx::query!(
            r#"
            INSERT INTO `markers` (`latitude`, `longtitude`, `title`, `description`,
            `type`, `add_time`, `end_time`, `address`, `contact_info`, `user_id`) 
            VALUES (?,?,?,?,?,?,?,?,?,?)"#,
            self.latitude,
            self.longtitude,
            self.title,
            self.description,
            self.event_type,
            chrono::offset::Utc::now(),
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
