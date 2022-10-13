pub use rocket::serde::json::Json;
pub use rocket_db_pools::{sqlx, Database};
pub use validator::Validate;

use serde::{Deserialize, Serialize};

use rand::Rng;

use bcrypt::{hash_with_salt, DEFAULT_COST};

#[derive(Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
pub struct UserLogin<'r> {
    #[validate(email)]
    email: &'r str,
    password: &'r str,
}

#[derive(Deserialize, Serialize, Validate)]
#[serde(crate = "rocket::serde")]
struct Address<'r> {
    street: &'r str,
    #[validate(length(min = 6, max = 6))]
    #[serde(rename = "postalCode")]
    postal_code: &'r str,
    country: &'r str,
    number: u32,
}

#[repr(u8)]
#[derive(Deserialize)]
enum Sex {
    Female = b'F',
    Male = b'M',
    Other = b'O',
}
impl Into<char> for Sex {
    fn into(self) -> char {
        self as u8 as char
    }
}
#[derive(Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
pub struct UserRegister<'r> {
    login: UserLogin<'r>,
    username: &'r str,
    name: &'r str,
    surname: &'r str,
    sex: char,
    #[validate]
    address: Address<'r>,
    reputation: u32,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct SomsiadStatus {
    status: String,
    errors: Vec<String>,
}
impl SomsiadStatus {
    pub fn errors(errors: Vec<String>) -> Json<Self> {
        Json(Self {
            errors: errors,
            status: "error".to_string(),
        })
    }

    pub fn error(error: &str) -> Json<Self> {
        Json(Self {
            errors: vec![error.to_string()],
            status: "error".to_string(),
        })
    }

    pub fn ok() -> Json<Self> {
        Json(Self {
            errors: Vec::new(),
            status: "ok".to_string(),
        })
    }
}
pub type JsonSomsiadStatus = Json<SomsiadStatus>;
pub type SomsiadResult<T> = Result<T, Json<SomsiadStatus>>;

impl UserLogin<'_> {
    pub async fn login(&self, db: &sqlx::MySqlPool) -> anyhow::Result<(bool, i32)> {
        let salt: String = sqlx::query_scalar("SELECT salt as salt FROM users WHERE email = ?")
            .bind(self.email)
            .fetch_one(db)
            .await?;

        let salt: [u8; 16] = salt.into_bytes().try_into().unwrap();
        let hashed_pass = hash_with_salt(self.password.as_bytes(), DEFAULT_COST, salt)?.to_string();

        let logged: Option<i32> =
            sqlx::query_scalar("SELECT id FROM users WHERE email = ? AND password = ?")
                .bind(self.email)
                .bind(hashed_pass)
                .fetch_optional(db)
                .await?;

        match logged {
            Some(row) => Ok((true, row)),
            None => Ok((false, 0)),
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

        let mut tx = db.begin().await?;

        let user_insert =
            sqlx::query("INSERT INTO users (email, name, password, salt) VALUES (?, ?, ?, ?);")
                .bind(self.login.email)
                .bind(self.username)
                .bind(hashed_pass.to_string())
                .bind(salt)
                .execute(&mut tx)
                .await?;
        let last_insert_id = user_insert.last_insert_id();

        let full_user_insert = sqlx::query("insert into full_users_info (id,name,surname,sex,address,reputation) values(?,?,?,?,?,?);")
            .bind(last_insert_id)
            .bind(self.name)
            .bind(self.surname)
            .bind(self.sex.to_string())
            .bind(serde_json::to_string(&self.address)?)
            .bind(self.reputation)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;

        let rows_affected = user_insert.rows_affected();
        Ok(rows_affected > 0)
    }
}
