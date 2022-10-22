use bcrypt::{hash_with_salt, DEFAULT_COST};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::login::UserLogin;

#[derive(Deserialize, Validate)]
pub struct UserRegister<'r> {
    #[validate]
    login: UserLogin<'r>,
    username: &'r str,
    name: &'r str,
    surname: &'r str,
    sex: Sex,
    #[validate]
    address: Address<'r>,
    #[serde(default)]
    reputation: u32,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct Address<'r> {
    street: &'r str,
    number: &'r str,
    city: &'r str,
}

#[derive(Serialize, Deserialize, sqlx::Type)]
pub(super) enum Sex {
    #[sqlx(rename = "F")]
    Female,
    #[sqlx(rename = "M")]
    Male,
    #[sqlx(rename = "O")]
    Other,
}

impl UserRegister<'_> {
    pub async fn add_to_db(&self, db: &sqlx::MySqlPool) -> anyhow::Result<bool> {
        let salt = nanoid!(16);
        let salt_copy: [u8; 16] = salt.as_bytes().try_into().unwrap();
        let hashed_pass = hash_with_salt(self.login.password.as_bytes(), DEFAULT_COST, salt_copy)?;

        let mut tx = db.begin().await?;

        let user_insert = sqlx::query!(
            "INSERT INTO users (email, name, password) VALUES (?, ?, ?);",
            self.login.email,
            self.username,
            hashed_pass.to_string(),
        )
        .execute(&mut tx)
        .await?;
        let last_insert_id = user_insert.last_insert_id();

        let full_user_insert = sqlx::query!(
            "insert into full_users_info (id,name,surname,sex,address,reputation) values(?,?,?,?,?,?);",
            last_insert_id,
            self.name,
            self.surname,
            self.sex,
            serde_json::to_string(&self.address)?,
            self.reputation)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;

        let rows_affected = user_insert.rows_affected();
        Ok(rows_affected == full_user_insert.rows_affected() && rows_affected > 0)
    }
}
