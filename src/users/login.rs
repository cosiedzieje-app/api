use bcrypt::{hash_with_salt, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::register::Sex;

#[derive(Deserialize, Validate)]
pub struct UserLogin<'r> {
    #[validate(email)]
    pub email: &'r str,
    pub password: &'r str,
}

#[derive(Serialize, Deserialize)]
pub struct AddressOwned {
    street: String,
    #[serde(rename = "postalCode")]
    postal_code: String,
    country: String,
    number: String,
}
#[derive(Serialize)]
pub struct UserPublicInfo {
    login_name: String,
    name: String,
    surname: String,
    email: String,
    sex: Sex,
    address: sqlx::types::Json<AddressOwned>,
    reputation: i32,
}

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
        .fetch_optional(db)
        .await?;

        match logged {
            Some(row) => Ok((true, row)),
            None => Ok((false, 0)),
        }
    }
}

impl UserPublicInfo {
    pub async fn from_id(db: &sqlx::MySqlPool, id: u32) -> anyhow::Result<Self> {
        let user = sqlx::query_as!(
            UserPublicInfo,
            r#"
        SELECT u.name as login_name, ext.name as name, ext.surname as surname, u.email as email, 
        ext.sex as `sex: Sex`, ext.address as `address: sqlx::types::Json<AddressOwned>`, ext.reputation as `reputation: i32`
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
