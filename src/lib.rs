/* modules */
pub mod markers;
pub mod routes;
pub mod users;
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

pub type SomsiadResult<T> = Json<SomsiadStatus<T>>;
