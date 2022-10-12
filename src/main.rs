pub mod structs;
use crate::structs::*;
use rocket::fs::relative;
use rocket::fs::FileServer;
use rocket::serde::json::Json;
#[macro_use]
extern crate rocket;

#[derive(Database)]
#[database("baza")]
pub struct Db(sqlx::MySqlPool);

#[post("/register", format = "json", data = "<user>")]
async fn register(db: &Db, user: Json<User<'_>>) -> JsonCustomError {
    match user.add_to_db(&db.0).await {
        Err(e) => match e.to_string().split(" ").last().unwrap_or_default() {
            "'email'" => CustomError::error("Provided email is already in use"),
            "'name'" => CustomError::error("Provided name is already in use"),
            _ => CustomError::error("Some data entered by you is wrond"),
        },
        Ok(false) => {
            warn_!("Zero rows affected, user not added");
            CustomError::error("Internal server error, we dont know what happened")
        }
        Ok(true) => {
            info_!("User added");
            CustomError::ok()
        }
    }
}

#[post("/login", data = "<user>")]
async fn login(db: &Db, user: Json<UserLogin<'_>>) -> JsonCustomError {
    match user.login(&db.0).await {
        Err(_) => CustomError::error("We fucked up"),
        Ok(false) => {
            warn_!("Invalid credentials");
            CustomError::error("Invalid credentials")
        }
        Ok(true) => {
            info_!("Logged Succesfully with id gonwo");
            CustomError::ok()
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
