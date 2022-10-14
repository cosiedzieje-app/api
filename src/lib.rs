/* modules */
pub mod markers;
pub mod routes;
pub mod users;
/* Uses */
pub use rocket::serde::json::Json;
use serde::Serialize;
pub use validator::Validate;

#[derive(Serialize)]
pub struct SomsiadStatus {
    status: String,
    errors: Vec<String>,
}
impl SomsiadStatus {
    pub fn errors(errors: Vec<String>) -> Json<Self> {
        Json(Self {
            errors,
            status: "error".to_string(),
        })
    }

    pub fn error(error: &str) -> Json<Self> {
        Json(Self {
            errors: vec![error.to_string()],
            status: "error".to_string(),
        })
    }

    pub fn ok() -> Json<Self> {
        Json(Self {
            errors: Vec::new(),
            status: "ok".to_string(),
        })
    }
}
pub type JsonSomsiadStatus = Json<SomsiadStatus>;
pub type SomsiadResult<T> = Result<T, Json<SomsiadStatus>>;
