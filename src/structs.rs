use std::ops::Add;

pub use rocket::serde::json::Json;
pub use rocket_db_pools::{sqlx, Database};

use anyhow;

use serde::{Deserialize, Serialize};

use rand::Rng;

use bcrypt::{DEFAULT_COST, hash_with_salt};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserLogin<'r> {
    name: &'r str,
    password: &'r str,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Address<'r> {
    street: &'r str,
    postalCode: &'r str,
    country: &'r str,
    number: u32,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserRegister<'r> {
    login: UserLogin<'r>,
    name: &'r str,
    surname: &'r str,
    email: &'r str,
    sex: char,
    address: Address<'r>,
    reputation: u32
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SomsiadStatus {
    status: String,
    error: String,
}
impl SomsiadStatus {
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
pub type JsonSomsiadStatus = Json<SomsiadStatus>;
pub type SomsiadResult<T> = Result<T, Json<SomsiadStatus>>;

impl UserLogin<'_> {
    pub async fn login(&self, db: &sqlx::MySqlPool) -> anyhow::Result<bool> {
        let salt: (String,) = sqlx::query_as("SELECT salt as salt FROM users WHERE name = ?")
            .bind(self.name)
            .fetch_one(db)
            .await?;

        let salt: [u8; 16] = salt.0.into_bytes().try_into().unwrap();
        let hashed_pass = hash_with_salt(self.password.as_bytes(), DEFAULT_COST, salt)?.to_string();

        let logged = sqlx::query("SELECT id FROM users WHERE name = ? AND password = ?")
            .bind(self.name)
            .bind(hashed_pass)
            .fetch_optional(db)
            .await?;

         match logged {
             Some(_) => Ok(true),
             None => Ok(false),
         }
    }
}

impl UserRegister<'_> {
    pub async fn add_to_db(&self, db: &sqlx::MySqlPool) -> anyhow::Result<bool> {
        //thread_rng is stored in thread local storage, so noticable performance impact is not expected
        let salt = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(16)
            .collect::<Vec<u8>>();
        let salt_copy: [u8; 16] = salt.clone().try_into().unwrap();
        let hashed_pass = hash_with_salt(self.login.password.as_bytes(), DEFAULT_COST, salt_copy)?;

        let rows_affected =
            sqlx::query("INSERT INTO users (email, name, password, salt) VALUES (?, ?, ?, ?)")
                .bind(self.email)
                .bind(self.login.name)
                .bind(hashed_pass.to_string())
                .bind(salt)
                .execute(db)
                .await?
                .rows_affected();

        Ok(rows_affected > 0)
    }
}
