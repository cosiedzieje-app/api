use bcrypt::verify;
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
    number: String,
    city: String,
}

#[derive(Serialize)]
pub struct UserPrivateInfo {
    username: String,
    name: String,
    surname: String,
    email: String,
    sex: Sex,
    address: sqlx::types::Json<AddressOwned>,
    reputation: i32,
}

#[derive(Serialize)]
pub struct UserPublicInfo {
    username: String,
    name: String,
    surname: String,
    sex: Sex,
    reputation: i32,
}

impl UserLogin<'_> {
    pub async fn login(&self, db: &sqlx::MySqlPool) -> anyhow::Result<(bool, i32)> {
        let user = sqlx::query!("SELECT password, id FROM users WHERE email = ?", self.email)
            .fetch_optional(db)
            .await?;

        let user = match user {
            Some(user) => user,
            None => return Ok((false, 0)),
        };

        // https://stackoverflow.com/questions/277044/do-i-need-to-store-the-salt-with-bcrypt
        let result = verify(self.password, &user.password).unwrap();

        Ok((result, user.id))
    }
}

impl UserPrivateInfo {
    pub async fn from_id(db: &sqlx::MySqlPool, id: u32) -> anyhow::Result<Self> {
        let user = sqlx::query_as!(
            UserPrivateInfo,
            r#"
        SELECT u.name as username, ext.name as name, ext.surname as surname, u.email as email, 
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

impl UserPublicInfo {
    pub async fn from_id(db: &sqlx::MySqlPool, id: u32) -> anyhow::Result<Self> {
        let user = sqlx::query_as!(
            UserPublicInfo,
            r#"
        SELECT u.name as username, ext.name as name, ext.surname as surname, 
        ext.sex as `sex: Sex`, ext.reputation as `reputation: i32`
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
