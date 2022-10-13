pub mod structs;
use crate::structs::*;
use nanoid::format;
use rocket::fs::relative;
use rocket::fs::FileServer;
use rocket::serde::json::Json;
use validator::ValidationErrors;
use rocket::http::{CookieJar, Cookie};
#[macro_use]
extern crate rocket;

#[derive(Database)]
#[database("somsiad")]
pub struct Db(sqlx::MySqlPool);

#[post("/register", format = "json", data = "<user>")]
async fn register(db: &Db, user: Json<UserRegister<'_>>) -> JsonSomsiadStatus {
    if let Err(e) = user.validate() {
        return SomsiadStatus::errors(e.errors().into_iter()
        .map(|(field, _)| field.to_string()).collect());
    }

    match user.add_to_db(&db.0).await {
        Err(e) => match e.to_string().split(" ").last().unwrap_or_default() {
            "'email'" => SomsiadStatus::error("Provided email is already in use"),
            "'name'" => SomsiadStatus::error("Provided name is already in use"),
            _ => {error!("Internal error: {}", e); SomsiadStatus::error("Some data entered by you is wrong")},
        },
        Ok(false) => {
            warn_!("Zero rows affected, user not added");
            SomsiadStatus::error("Internal server error, we dont know what happened")
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
            SomsiadStatus::error("Unexpected error occured during login!")
        },
        Ok((false, _)) => {
            SomsiadStatus::error("Invalid credentials")
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
