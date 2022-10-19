use dotenv::dotenv;
use rocket::fs::relative;
use rocket::fs::FileServer;
use somsiad_api::routes::*;
use sqlx::pool::PoolOptions;
use sqlx::MySql;
use std::env;
use somsiad_api::fairings;
use somsiad_api::catchers;
use figment::{Figment, providers::{Format, Toml}};
use rocket::fairing::AdHoc;
use somsiad_api::Config;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();
    let db = PoolOptions::<MySql>::new()
        .min_connections(0)
        .max_connections(10)
        .test_before_acquire(true)
        .connect(&env::var("DATABASE_URL").expect("Failed to acquire DB URL"))
        .await
        .expect("Failed to connect to db");

    let config = Figment::from(rocket::Config::default())
        .merge(Toml::file("Rocket.toml").nested());

    let _rocket = rocket::custom(config)
        .attach(fairings::CORS)
        .attach(AdHoc::config::<Config>())
        .manage(db)
        .mount(
            "/",
            routes![
                login,
                register,
                logout,
                get_markers,
                add_marker,
                get_marker,
                remove_marker,
                user_data,
                get_user_markers,
            ],
        )
        .mount("/", FileServer::from(relative!("static")))
        .register("/", catchers![
                  catchers::options_catcher
        ])
        .launch()
        .await?;

    Ok(())
}
