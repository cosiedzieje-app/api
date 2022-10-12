pub use rocket::serde::json::Json;
use rocket_db_pools::sqlx::{query, Row};
pub use rocket_db_pools::{sqlx, Database};

use anyhow;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserLogin<'r> {
    name: &'r str,
    password: &'r str,
}
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User<'r> {
    login: UserLogin<'r>,
    email: &'r str,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CustomError {
    status: String,
    error: String,
}
impl CustomError {
    pub fn error(error: &str) -> Json<Self> {
        Json(Self {
            error: error.to_string(),
            status: "error".to_string(),
        })
    }

    pub fn ok() -> Json<Self> {
        Json(Self {
            error: "".to_string(),
            status: "ok".to_string(),
        })
    }
}
pub type JsonCustomError = Json<CustomError>;
pub type CustomResult<T> = Result<T, Json<CustomError>>;
impl UserLogin<'_> {
    pub async fn login(&self, db: &sqlx::MySqlPool) -> anyhow::Result<bool> {
        let salt = sqlx::query!("SELECT salt as salt FROM users WHERE name = ?")
            .bind(self.name)
            .fetch_optional(db)
            .await?
            .salt;
        let logged = sqlx::query("SELECT id FROM users WHERE name = ? AND password = ?")
            .bind(self.name)
            .bind(self.password)
            .fetch_optional(db)
            .await?;

        match logged {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
}

impl User<'_> {
    pub async fn add_to_db(&self, db: &sqlx::MySqlPool) -> anyhow::Result<bool> {
        let rows_affected =
            sqlx::query("INSERT INTO users (email, name, password,salt) VALUES (?, ?, ?,?)")
                .bind(self.email)
                .bind(self.login.name)
                .bind(self.login.password)
                .bind("sul")
                .execute(db)
                .await?
                .rows_affected();

        Ok(rows_affected > 0)
    }
}
