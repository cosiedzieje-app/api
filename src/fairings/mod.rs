use rocket::http::Header;
use rocket::{Request, Response, State};
use rocket::fairing::{Fairing, Info, Kind};
use crate::{Config, Mode};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "CORS",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        let config = request.guard::<&State<Config>>()
            .await
            .unwrap()
            .inner();

        //NOTE: This is borked. It should not have been done.
        //But it was. Oh well.
        if config.mode == Mode::Debug {
            response.set_header(Header::new("Access-Control-Allow-Origin", "http://localhost:5173"));
        } else {
            //TODO: Replace with actual domain
            response.set_header(Header::new("Access-Control-Allow-Origin", "https://localhost:5173"));
        }

        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, DELETE, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "Accept, Content-Type"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
