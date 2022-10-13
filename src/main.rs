pub mod structs;
use crate::structs::*;
use nanoid::format;
use rocket::fs::relative;
use rocket::fs::FileServer;
use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::Json;
use validator::ValidationErrors;
#[macro_use]
extern crate rocket;

#[derive(Database)]
#[database("somsiad")]
pub struct Db(sqlx::MySqlPool);

#[get("/markers")]
async fn get_markers(db: &Db) -> Json {}

#[post("/register", format = "json", data = "<user>")]
async fn register(db: &Db, user: Json<UserRegister<'_>>) -> JsonSomsiadStatus {
    if let Err(e) = user.validate() {
        return SomsiadStatus::errors(
            e.errors()
                .into_iter()
                .map(|(field, _)| field.to_string())
                .collect(),
        );
    }

    match user.add_to_db(&db.0).await {
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
async fn login(db: &Db, cookies: &CookieJar<'_>, user: Json<UserLogin<'_>>) -> JsonSomsiadStatus {
    match user.login(&db.0).await {
        Err(e) => {
            error!("Internal error: {}", e);
            SomsiadStatus::error("Nieoczekiwany błąd podczas logowania")
        }
        Ok((false, _)) => {
            SomsiadStatus::error("Nick lub hasło podane przez ciebie nie są poprawne")
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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![login, register, logout])
        .mount("/", FileServer::from(relative!("static")))
}
