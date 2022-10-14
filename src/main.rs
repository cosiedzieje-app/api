pub mod markers;
pub mod structs;
use crate::structs::*;
use dotenv::dotenv;
use markers::show_markers;
use rocket::fs::relative;
use rocket::fs::FileServer;
use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::Json;
use sqlx::pool::PoolOptions;
use sqlx::MySql;
use sqlx::MySqlPool;
use std::env;
#[macro_use]
extern crate rocket;

/* impl<'r> Responder<'r, 'r> for Vec<Json<Marker>> {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        Response::build().sized_body(Stream::from(Cursor::new(self)))
    }
} */

#[get("/markers")]
async fn get_markers(db: &rocket::State<MySqlPool>) -> String {
    show_markers(db).await.unwrap()
}

#[post("/register", format = "json", data = "<user>")]
async fn register(
    db: &rocket::State<MySqlPool>,
    user: Json<UserRegister<'_>>,
) -> JsonSomsiadStatus {
    if let Err(e) = user.validate() {
        return SomsiadStatus::errors(
            e.errors()
                .into_iter()
                .map(|(field, _)| field.to_string())
                .collect(),
        );
    }

    match user.add_to_db(&db).await {
        Err(e) => match e.to_string().split(" ").last().unwrap_or_default() {
            "'email'" => SomsiadStatus::error("Podany e-mail jest zajęty"),
            "'name'" => SomsiadStatus::error("Podany nick jest zajęty"),
            _ => {
                error!("Internal error: {}", e);
                SomsiadStatus::error("Nieoczekiwany błąd")
            }
        },
        Ok(false) => {
            warn_!("Zero rows affected, user not added");
            SomsiadStatus::error("Nieoczekiwany błąd")
        }
        Ok(true) => {
            info_!("User added");
            SomsiadStatus::ok()
        }
    }
}

#[post("/login", data = "<user>")]
async fn login(
    db: &rocket::State<MySqlPool>,
    cookies: &CookieJar<'_>,
    user: Json<UserLogin<'_>>,
) -> JsonSomsiadStatus {
    match user.login(&db).await {
        Err(e) => {
            error!("Internal error: {}", e);
            SomsiadStatus::error("Nieoczekiwany błąd podczas logowania")
        }
        Ok((false, _)) => {
            SomsiadStatus::error("Email lub hasło podane przez ciebie nie są poprawne")
        }
        Ok((true, id)) => {
            info_!("Logged Succesfully with id: {}", id);
            cookies.add_private(Cookie::new("id", id.to_string()));
            SomsiadStatus::ok()
        }
    }
}

#[get("/logout")]
async fn logout(cookies: &CookieJar<'_>) -> JsonSomsiadStatus {
    cookies.remove_private(Cookie::named("id"));
    SomsiadStatus::ok()
}

#[get("/user_data")]
async fn user_data<'a>(
    db: &rocket::State<MySqlPool>,
    cookies: &CookieJar<'_>,
) -> SomsiadResult<Json<UserPublicInfo>> {
    match cookies.get_private("id") {
        Some(cookie) => {
            let id: i32 = cookie.value().parse().unwrap_or_default();
            if id == 0 {
                Err(SomsiadStatus::error(
                    "Twój token logowania jest nieprawidłowy!",
                ))
            } else {
                match UserPublicInfo::from_id(&db, id).await {
                    Ok(user) => Ok(Json(user)),
                    Err(e) => {
                        error!("Internal error: {}", e);
                        Err(SomsiadStatus::error("Wewnętrzny błąd"))
                    }
                }
            }
        }
        None => Err(SomsiadStatus::error("Nie jesteś zalogowany")),
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();
    let db_url = "mysql://cursor:s0ms1a$@localhost/somsiad";
    let db = PoolOptions::<MySql>::new()
        .min_connections(0)
        .max_connections(10)
        .test_before_acquire(true)
        .connect(
            db_url, /* &env::var("DATABASE_URL").expect("Failed to acquire DB URL") */
        )
        .await
        .expect("Failed to connect to db");

    let _rocket = rocket::build()
        .mount(
            "/",
            routes![login, register, logout, get_markers, user_data],
        )
        .mount("/", FileServer::from(relative!("static")))
        .manage(db)
        .launch()
        .await?;

    Ok(())
}
