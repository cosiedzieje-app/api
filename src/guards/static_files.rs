use rocket::request::{FromRequest, Request, Outcome};

pub struct StaticFiles;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for StaticFiles {
    type Error = ();

    async fn from_request(_req: &'r Request<'_>) 
    -> Outcome<Self, Self::Error> {
        Outcome::Forward(())
    } 
}
