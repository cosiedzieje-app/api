/* modules */
pub mod markers;
pub mod routes;
pub mod users;
pub mod fairings;
pub mod catchers;
use rocket::http::Cookie;
/* Uses */
pub use rocket::serde::json::Json;
use serde::Serialize;
pub use validator::Validate;

#[derive(Serialize)]
#[serde(tag = "status", content = "res")]
pub enum SomsiadStatus<T> {
    #[serde(rename = "ok")]
    Ok(T),
    #[serde(rename = "error")]
    Error(Vec<String>),
}

impl<T> SomsiadStatus<T> {
    pub fn errors(errors: Vec<String>) -> Json<Self> {
        Json(Self::Error(errors))
    }

    pub fn error(error: &str) -> Json<Self> {
        Json(Self::Error(vec![error.to_string()]))
    }

    pub fn ok(obj: T) -> Json<Self> {
        Json(Self::Ok(obj))
    }
}

pub fn validate_id_cookie(id: Option<Cookie>) -> SomsiadResult<u32> {
    match id {
        Some(cookie) => match cookie.value().parse().unwrap_or_default() {
            0 => SomsiadStatus::error("Twój token logowania jest nieprawidłowy"),
            val @ _ => SomsiadStatus::ok(val),
        },
        None => SomsiadStatus::error("Nie jesteś zalogowany"),
    }
}

pub type SomsiadResult<T> = Json<SomsiadStatus<T>>;
