use std::path::Path;
use rocket::{
    http::Status,
    request::{FromRequest, Request, Outcome},
    tokio::fs,
    fs::relative
};

#[derive(Debug)]
pub enum MaintenanceMode {
    Enabled,
    Disabled
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for MaintenanceMode {
    type Error = MaintenanceMode;

    async fn from_request(_req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match fs::metadata(Path::new(relative!("maintenance"))).await {
            Ok(_) => Outcome::Failure((
                    Status::ServiceUnavailable,
                    MaintenanceMode::Enabled
            )),
            Err(_) => Outcome::Success(MaintenanceMode::Disabled)
        } 
    }
}
