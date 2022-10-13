pub mod structs;
use crate::structs::*;
use rocket::fs::relative;
use rocket::fs::FileServer;
use rocket::log::private::info;
use rocket::serde::json::Json;
#[macro_use]
extern crate rocket;

#[derive(Database)]
#[database("somsiad")]
pub struct Db(sqlx::MySqlPool);

#[post("/register", format = "json", data = "<user>")]
async fn register(db: &Db, user: Json<UserRegister<'_>>) -> JsonSomsiadStatus {
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
async fn login(db: &Db, user: Json<UserLogin<'_>>) -> JsonSomsiadStatus {
    match user.login(&db.0).await {
        Err(e) => {error!("Internal error: {}", e); SomsiadStatus::error("Unexpected error occured during login!")},
        Ok(false) => {
            warn_!("Invalid credentials");
            SomsiadStatus::error("Invalid credentials")
        }
        Ok(true) => {
            info_!("Logged Succesfully with id: (some id)");
            SomsiadStatus::ok()
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![login, register])
        .mount("/", FileServer::from(relative!("static")))
}
