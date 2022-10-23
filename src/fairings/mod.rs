use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "CORS",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _: &'r Request<'_>, response: &mut Response<'r>) {
        //NOTE: This is borked. It should not have been done.
        //But it was. Oh well.
        //NOTE2: no need to bloat program with unnecessary crates
        //when you need just one feature of them
        if cfg!(debug_assertions) {
            response.set_header(Header::new(
                "Access-Control-Allow-Origin",
                "http://localhost:5173",
            ));
        } else {
            response.set_header(Header::new(
                "Access-Control-Allow-Origin",
                "https://cosiedzieje.mikut.dev",
            ));
        }

        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, DELETE, OPTIONS, PUT",
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Headers",
            "Accept, Content-Type",
        ));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
