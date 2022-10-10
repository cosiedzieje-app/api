#[macro_use] extern crate rocket;

use rocket::{form::Form, fs::FileServer};
use rocket::http::Status;

use rocket_db_pools::{sqlx, Database};

use anyhow;

use rocket::fs::relative;

#[derive(Database)]
#[database("baza")]
struct Db(sqlx::MySqlPool);

#[derive(FromForm)]
struct User<'r> {
    email: &'r str,
    name: &'r str,
    password: &'r str,
}
// CREATE DATABASE baza;
// CREATE TABLE `users` (
//  `id` int(11) NOT NULL AUTO_INCREMENT,
//  `email` varchar(255) NOT NULL,
//  `name` varchar(255) NOT NULL,
//  `password` varchar(255) NOT NULL,
//  PRIMARY KEY (`id`)
// ) DEFAULT CHARSET=utf8mb4;
impl User<'_>{
    async fn add(&self, db: &sqlx::MySqlPool) -> anyhow::Result<bool> {
        let rows_affected = sqlx::query("INSERT INTO users (email, name, password) VALUES (?, ?, ?)")
            .bind(self.email)
            .bind(self.name)
            .bind(self.password)
            .execute(db)
            .await?
            .rows_affected();

        Ok(rows_affected > 0)
    }
}

#[post("/register", data = "<user>")]
async fn register(db: &Db, user: Form<User<'_>>) -> Status {
    match user.add(&db.0).await{
        Err(e) => {error_!("Internal server error: {}", e); Status::InternalServerError},
        Ok(false) =>{ info_!("Zero rows affected, user not added"); Status::InternalServerError},
        Ok(true) => { info_!("User added"); Status::Ok },
    }
}

#[post("/login", data = "<user>")]
fn login(db: &Db, user: Form<User<'_>>) {

}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![login, register])
        .mount("/", FileServer::from(relative!("static")))
}