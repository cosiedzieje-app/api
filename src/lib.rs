/* modules */
pub mod fairings;
pub mod markers;
pub mod routes;
pub mod users;

use rocket::{http::Status, outcome::IntoOutcome, request, Request};
/* Uses */
pub use rocket::config::SecretKey;
use rocket::request::FromRequest;
pub use rocket::serde::json::Json;
use serde::Serialize;
pub use validator::Validate;

#[derive(Serialize, Debug)]
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

pub struct UserID(u32);
impl From<u32> for UserID {
    fn from(val: u32) -> Self {
        Self(val)
    }
}
#[rocket::async_trait]
impl<'a> FromRequest<'a> for UserID {
    type Error = ();

    async fn from_request(request: &'a Request<'_>) -> request::Outcome<Self, Self::Error> {
        request
            .cookies()
            .get_private("id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|id| Self(id))
            .into_outcome((Status::Unauthorized, ()))
    }
}
