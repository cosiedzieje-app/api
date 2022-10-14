use dotenv::dotenv;
use rocket::fs::relative;
use rocket::fs::FileServer;
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
        .max_connections(10)
        .test_before_acquire(true)
        .connect(&env::var("DATABASE_URL").expect("Failed to acquire DB URL"))
        .await
        .expect("Failed to connect to db");

    let _rocket = rocket::build()
        .mount(
            "/",
            routes![
                login,
                register,
                logout,
                get_markers,
                add_marker,
                get_marker,
                user_data
            ],
        )
        .mount("/", FileServer::from(relative!("static")))
        .manage(db)
        .launch()
        .await?;

    Ok(())
}
