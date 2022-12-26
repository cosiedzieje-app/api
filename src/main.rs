use rocket::fs::{FileServer, relative};
use dotenv::dotenv;
use somsiad_api::fairings;
use somsiad_api::routes;
use sqlx::pool::PoolOptions;
use sqlx::MySql;
use std::env;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();
    let db = PoolOptions::<MySql>::new()
        .min_connections(0)
        .max_connections(500)
        .test_before_acquire(true)
        .connect(&env::var("DATABASE_URL").expect("Failed to acquire DB URL"))
        .await
        .expect("Failed to connect to db");

    let _rocket = rocket::build()
        .attach(fairings::CORS)
        .manage(db)
        .register(
            "/",
            catchers![
                routes::root::spa_catcher,
                routes::root::maintenance_catch,
                routes::root::default_catcher
            ]
        )
        .register(
            "/api", 
            catchers![
                routes::api::options_catcher, 
                routes::api::unauthorized_catcher,
                routes::api::maintenance_catcher
            ]
        )
        .register(
            "/positionstack",
            catchers![
                routes::positionstack::default_catcher,
                routes::positionstack::maintenance_catcher
            ]
        )
        .mount(
            "/", 
            routes![
                routes::root::get_page
            ]
        )
        .mount(
            "/",
            FileServer::from(relative!("static"))
        )
        .mount(
            "/api",
            routes![
                routes::api::login,
                routes::api::register,
                routes::api::logout,
                routes::api::get_user_data,
                routes::api::user_data,
                routes::api::is_logged,
                routes::api::get_markers,
                routes::api::add_marker,
                routes::api::remove_marker,
                routes::api::get_user_markers,
                routes::api::get_markers_by_city,
                routes::api::get_markers_by_dist,
            ],
        )
        .mount(
            "/positionstack",
            routes![
                routes::positionstack::get_positionstack
            ]
        )
        .launch()
        .await?;

    Ok(())
}
