use crate::markers::*;
use crate::users::login::*;
use crate::users::register::*;
use crate::*;
use crate::{SomsiadResult, SomsiadStatus};
use rocket::{
    catch, delete, error_, get,
    http::{Cookie, CookieJar, Method, Status},
    info_, post, put,
    serde::json::Json,
    warn_, Request,
};
use sqlx::MySqlPool;
use validator::ValidationErrorsKind::*;

#[catch(401)]
pub fn unauthorized_catcher() -> SomsiadResult<&'static str> {
    SomsiadStatus::error("Nie jesteś zalogowany")
}

#[catch(404)]
pub fn options_catcher<'a>(status: Status, request: &Request) -> (Status, SomsiadResult<&'a str>) {
    if request.method() == Method::Options {
        (Status::Ok, SomsiadStatus::ok(""))
    } else {
        (
            status,
            SomsiadStatus::error(format!("Ścieżka {} nie istnieje!", request.uri()).as_str()),
        )
    }
}

#[get("/is_logged")]
pub async fn is_logged(_user: UserID) -> SomsiadResult<&'static str> {
    SomsiadStatus::ok("Jesteś zalogowany")
}

#[get("/user_markers")]
pub async fn get_user_markers(
    db: &rocket::State<MySqlPool>,
    user_id: UserID,
) -> SomsiadResult<Vec<FullMarkerOwned>> {
    match show_user_markers(db, user_id.0).await {
        Ok(markers) => SomsiadStatus::ok(markers),
        Err(e) => {
            error_!("Error: {}", e);
            SomsiadStatus::error("Wewnętrzny błąd serwera")
        }
    }
}

#[get("/markers/<city>", rank = 2)]
pub async fn get_markers_by_city(
    db: &rocket::State<MySqlPool>,
    city: &str,
) -> SomsiadResult<Vec<FullMarkerOwned>> {
    match show_markers_by_city(db, city).await {
        Ok(markers) => SomsiadStatus::ok(markers),
        Err(e) => {
            error_!("Error: {}", e);
            SomsiadStatus::error("Wewnętrzny błąd serwera")
        }
    }
}

#[get("/markers?<lat>&<long>&<dist>")]
pub async fn get_markers_by_dist(
    db: &rocket::State<MySqlPool>,
    lat: f64,
    long: f64,
    dist: u32,
) -> SomsiadResult<Vec<FullMarkerOwnedWithDist>> {
    match show_markers_by_dist(db, lat, long, dist).await {
        Ok(markers) => SomsiadStatus::ok(markers),
        Err(e) => {
            error_!("Error: {}", e);
            SomsiadStatus::error("Wewnętrzny błąd serwera")
        }
    }
}
#[get("/markers")]
pub async fn get_markers(db: &rocket::State<MySqlPool>) -> SomsiadResult<Vec<FullMarkerOwned>> {
    match show_markers(db).await {
        Ok(markers) => SomsiadStatus::ok(markers),
        Err(e) => {
            error_!("Error: {}", e);
            SomsiadStatus::error("Wewnętrzny błąd serwera")
        }
    }
}

#[put("/markers", format = "json", data = "<marker>")]
pub async fn add_marker(
    db: &rocket::State<MySqlPool>,
    marker: Json<FullMarker<'_>>,
    user_id: UserID,
) -> SomsiadResult<()> {
    match marker.add_marker(db, user_id.0).await {
        Err(e) => {
            error_!("Internal error: {}", e);
            SomsiadStatus::error("Nieoczekiwany błąd")
        }
        Ok(false) => {
            warn_!("Zero rows affected, user not added");
            SomsiadStatus::error("Nieoczekiwany błąd")
        }
        Ok(true) => SomsiadStatus::ok(()),
    }
}

#[delete("/markers/<marker_id>")]
pub async fn remove_marker(
    db: &rocket::State<MySqlPool>,
    user_id: UserID,
    marker_id: u32,
) -> SomsiadResult<FullMarkerOwned> {
    match delete_marker(db, user_id.0, marker_id).await {
        Err(e) => {
            error_!("Error in remove_marker: {}", e);
            SomsiadStatus::error("Nieoczekiwany błąd")
        }
        Ok(marker) => SomsiadStatus::ok(marker),
    }
}

#[post("/register", format = "json", data = "<user>")]
pub async fn register(
    db: &rocket::State<MySqlPool>,
    user: Json<UserRegister<'_>>,
) -> SomsiadResult<()> {
    if let Err(e) = user.validate() {
        return SomsiadStatus::errors(
            e.errors()
                .iter()
                .map(|(field, err_kinds)| match err_kinds {
                    Struct(err) => err
                        .errors()
                        .iter()
                        .map(|(field, _)| field.to_string())
                        .collect(),
                    _ => field.to_string(),
                })
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
            SomsiadStatus::ok(())
        }
    }
}

#[post("/login", data = "<user>")]
pub async fn login(
    db: &rocket::State<MySqlPool>,
    cookies: &CookieJar<'_>,
    user: Json<UserLogin<'_>>,
) -> SomsiadResult<()> {
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
            SomsiadStatus::ok(())
        }
    }
}

#[get("/logout")]
pub async fn logout(cookies: &CookieJar<'_>) -> SomsiadResult<()> {
    cookies.remove_private(Cookie::named("id"));
    SomsiadStatus::ok(())
}

#[get("/user_data")]
pub async fn user_data(
    db: &rocket::State<MySqlPool>,
    user_id: UserID,
) -> SomsiadResult<UserPrivateInfo> {
    match UserPrivateInfo::from_id(db, user_id.0).await {
        Ok(user) => SomsiadStatus::ok(user),
        Err(e) => {
            error_!("Internal error: {}", e);
            SomsiadStatus::error("Wewnętrzny błąd")
        }
    }
}

#[get("/user/<id>")]
pub async fn get_user_data(
    db: &rocket::State<MySqlPool>,
    id: u32,
) -> SomsiadResult<UserPublicInfo> {
    match UserPublicInfo::from_id(db, id).await {
        Ok(user) => SomsiadStatus::ok(user),
        Err(e) => {
            error_!("Internal error: {}", e);
            SomsiadStatus::error("Wewnętrzny błąd")
        }
    }
}
