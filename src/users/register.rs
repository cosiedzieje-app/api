use bcrypt::{hash_with_salt, DEFAULT_COST};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use super::login::UserLogin;

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

#[derive(Deserialize, Serialize, Validate)]
pub struct Address<'r> {
    street: &'r str,
    #[validate(custom = "validate_postal_code")]
    #[serde(rename = "postalCode")]
    postal_code: &'r str,
    country: &'r str,
    number: u32,
}

#[repr(u8)]
#[derive(Serialize, Deserialize, sqlx::Type)]
pub(super) enum Sex {
    Female = b'F',
    Male = b'M',
    Other = b'O',
}

impl Into<char> for Sex {
    fn into(self) -> char {
        self as u8 as char
    }
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
            .execute(&mut tx)
            .await?;

        tx.commit().await?;

        let rows_affected = user_insert.rows_affected();
        Ok(rows_affected == full_user_insert.rows_affected() && rows_affected > 0)
    }
}
