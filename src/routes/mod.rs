use crate::markers::*;
use crate::users::login::*;
use crate::users::register::*;
use crate::*;
use rocket::{
    error_, get,
    http::{Cookie, CookieJar},
    info_, post,
    serde::json::Json,
    warn_,
};
use sqlx::MySqlPool;

#[get("/markers/<id>")]
pub async fn get_marker(
    db: &rocket::State<MySqlPool>,
    id: u32,
) -> SomsiadResult<Json<FullMarkerOwned>> {
    match show_marker(db, id).await {
        Ok(marker) => Ok(Json(marker)),
        Err(_) => {
            // error_!("Error: {}", e);
            Err(SomsiadStatus::error("Invalid ID"))
        }
    }
}

#[get("/markers")]
pub async fn get_markers(db: &rocket::State<MySqlPool>) -> SomsiadResult<Json<Vec<Marker>>> {
    match show_markers(db).await {
        Ok(markers) => Ok(Json(markers)),
        Err(e) => {
            error_!("Error: {}", e);
            Err(SomsiadStatus::error("Wewnętrzny błąd serwera"))
        }
    }
}

#[post("/add_marker", format = "json", data = "<marker>")]
pub async fn add_marker(
    db: &rocket::State<MySqlPool>,
    marker: Json<FullMarker<'_>>,
) -> JsonSomsiadStatus {
    match marker.add_marker(db).await {
        Err(e) => {
            error_!("Internal error: {}", e);
            SomsiadStatus::error("Nieoczekiwany błąd")
        }
        Ok(false) => {
            warn_!("Zero rows affected, user not added");
            SomsiadStatus::error("Nieoczekiwany błąd")
        }
        Ok(true) => SomsiadStatus::ok(),
    }
}

#[post("/register", format = "json", data = "<user>")]
pub async fn register(
    db: &rocket::State<MySqlPool>,
    user: Json<UserRegister<'_>>,
) -> JsonSomsiadStatus {
    if let Err(e) = user.validate() {
        return SomsiadStatus::errors(
            e.errors()
                .iter()
                .map(|(field, _)| field.to_string())
                .collect(),
        );
    }

    match user.add_to_db(db).await {
        Err(e) => match e.to_string().split(' ').last().unwrap_or_default() {
            "'email'" => SomsiadStatus::error("Podany e-mail jest zajęty"),
            "'name'" => SomsiadStatus::error("Podany nick jest zajęty"),
            _ => {
                error_!("Internal error: {}", e);
                SomsiadStatus::error("Nieoczekiwany błąd")
            }
        },
        Ok(false) => {
            warn_!("Zero rows affected, user not added");
            SomsiadStatus::error("Nieoczekiwany błąd")
        }
        Ok(true) => {
            info_!("User added");
            SomsiadStatus::ok()
        }
    }
}

#[post("/login", data = "<user>")]
pub async fn login(
    db: &rocket::State<MySqlPool>,
    cookies: &CookieJar<'_>,
    user: Json<UserLogin<'_>>,
) -> JsonSomsiadStatus {
    match user.login(db).await {
        Err(e) => {
            error_!("Internal error: {}", e);
            SomsiadStatus::error("Nieoczekiwany błąd podczas logowania")
        }
        Ok((false, _)) => {
            SomsiadStatus::error("Email lub hasło podane przez ciebie nie są poprawne")
        }
        Ok((true, id)) => {
            info_!("Logged Succesfully with id: {}", id);
            cookies.add_private(Cookie::new("id", id.to_string()));
            SomsiadStatus::ok()
        }
    }
}

#[get("/logout")]
pub async fn logout(cookies: &CookieJar<'_>) -> JsonSomsiadStatus {
    cookies.remove_private(Cookie::named("id"));
    SomsiadStatus::ok()
}

#[get("/user_data")]
pub async fn user_data<'a>(
    db: &rocket::State<MySqlPool>,
    cookies: &CookieJar<'_>,
) -> SomsiadResult<Json<UserPublicInfo>> {
    match cookies.get_private("id") {
        Some(cookie) => {
            let id: i32 = cookie.value().parse().unwrap_or_default();
            if id == 0 {
                Err(SomsiadStatus::error(
                    "Twój token logowania jest nieprawidłowy!",
                ))
            } else {
                match UserPublicInfo::from_id(db, id).await {
                    Ok(user) => Ok(Json(user)),
                    Err(e) => {
                        error_!("Internal error: {}", e);
                        Err(SomsiadStatus::error("Wewnętrzny błąd"))
                    }
                }
            }
        }
        None => Err(SomsiadStatus::error("Nie jesteś zalogowany")),
    }
}
