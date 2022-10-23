use dotenv::dotenv;
use rocket::fs::relative;
use rocket::fs::FileServer;
use somsiad_api::fairings;
use somsiad_api::routes::*;
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
        .mount("/", FileServer::from(relative!("static")).rank(1))
        .mount(
            "/api",
            routes![
                login,
                register,
                logout,
                get_user_data,
                user_data,
                is_logged,
                get_markers,
                add_marker,
                remove_marker,
                get_user_markers,
                get_markers_by_city,
                get_markers_by_dist,
            ],
        )
        .register("/", catchers![options_catcher, unauthorized_catcher])
        .launch()
        .await?;

    Ok(())
}
