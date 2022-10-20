use rocket::{catch, Request};
use rocket::http::{Status, Method};
use crate::{SomsiadResult, SomsiadStatus};

#[catch(404)]
pub fn options_catcher<'a>(status: Status, request: &Request) -> (Status, SomsiadResult<&'a str>) {
    if request.method() == Method::Options {
        (Status::Ok, SomsiadStatus::ok("")) 
    } else {
        (status, SomsiadStatus::error(format!("Ścieżka {} nie istnieje!", request.uri()).as_str()))
    }
}
