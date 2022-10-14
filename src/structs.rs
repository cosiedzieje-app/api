pub use rocket::serde::json::Json;
pub use validator::Validate;

use serde::{Deserialize, Serialize};

use bcrypt::{hash_with_salt, DEFAULT_COST};

use nanoid::nanoid;
use validator::ValidationError;

#[derive(Deserialize, Validate)]
pub struct UserLogin<'r> {
    #[validate(email)]
    email: &'r str,
    password: &'r str,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct Address<'r> {
    street: &'r str,
    #[validate(custom = "validate_postal_code")]
    #[serde(rename = "postalCode")]
    postal_code: &'r str,
    country: &'r str,
    number: u32,
}

fn validate_postal_code(postal_code: &str) -> Result<(), ValidationError> {
    if postal_code.len() != 6 {
        return Err(ValidationError::new("bad_postal_code"));
    }

    let splits: Vec<&str> = postal_code.split("-").collect();
    if splits.len() != 2 || splits[0].len() != 2 || splits[1].len() != 3 {
        return Err(ValidationError::new("bad_postal_code"));
    }

    if !splits[0].chars().all(char::is_numeric) || !splits[1].chars().all(char::is_numeric) {
        return Err(ValidationError::new("bad_postal_code"));
    }

    Ok(())
}

#[repr(u8)]
#[derive(Serialize, Deserialize, sqlx::Type)]
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
pub struct UserPublicInfo {
    login_name: String,
    name: String,
    surname: String,
    email: String,
    sex: Sex,
    address: String,
    reputation: i32,
}

impl UserPublicInfo {
    pub async fn from_id(db: &sqlx::MySqlPool, id: i32) -> anyhow::Result<Self> {
        let user = sqlx::query_as!(
            UserPublicInfo,
            r#"
        SELECT u.name as login_name, ext.name as name, ext.surname as surname, u.email as email, 
        ext.sex as `sex: Sex`, ext.address as address, ext.reputation as `reputation: i32`
        FROM users as u 
        INNER JOIN full_users_info as ext ON u.id = ext.id
        WHERE u.id = ?"#,
            id
        )
        .fetch_one(db)
        .await?;

        Ok(user)
    }
}

#[derive(Serialize)]
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
        let salt: Option<String> =
            sqlx::query_scalar!("SELECT salt as salt FROM users WHERE email = ?", self.email)
                .fetch_optional(db)
                .await?;

        let salt = match salt {
            Some(salt) => salt,
            None => return Ok((false, 0)),
        };

        let salt: [u8; 16] = salt.into_bytes().try_into().unwrap();
        let hashed_pass = hash_with_salt(self.password.as_bytes(), DEFAULT_COST, salt)?.to_string();

        let logged: Option<i32> = sqlx::query_scalar!(
            "SELECT id FROM users WHERE email = ? AND password = ?",
            self.email,
            hashed_pass
        )
        /* .bind(self.email)
        .bind(hashed_pass) */
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
        let salt = nanoid!(16);
        let salt_copy: [u8; 16] = salt.as_bytes().clone().try_into().unwrap();
        let hashed_pass = hash_with_salt(self.login.password.as_bytes(), DEFAULT_COST, salt_copy)?;

        let mut tx = db.begin().await?;

        let user_insert = sqlx::query!(
            "INSERT INTO users (email, name, password, salt) VALUES (?, ?, ?, ?);",
            self.login.email,
            self.username,
            hashed_pass.to_string(),
            salt
        )
        /* .bind(self.login.email)
        .bind(self.username)
        .bind(hashed_pass.to_string())
        .bind(salt) */
        .execute(&mut tx)
        .await?;
        let last_insert_id = user_insert.last_insert_id();

        let full_user_insert = sqlx::query!(
            "insert into full_users_info (id,name,surname,sex,address,reputation) values(?,?,?,?,?,?);",
            last_insert_id,
            self.name,
            self.surname,
            self.sex.to_string(),
            serde_json::to_string(&self.address)?,
            self.reputation)
            /* .bind(last_insert_id)
            .bind(self.name)
            .bind(self.surname)
            .bind(self.sex.to_string())
            .bind(serde_json::to_string(&self.address)?)
            .bind(self.reputation) */
            .execute(&mut tx)
            .await?;

        tx.commit().await?;

        let rows_affected = user_insert.rows_affected();
        Ok(rows_affected == full_user_insert.rows_affected() && rows_affected > 0)
    }
}
