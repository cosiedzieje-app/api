use std::path::{PathBuf, Path};
use rocket::{
    get,
    catch,
    http::Status,
    fs::{NamedFile, relative},
    Request
};
use crate::{
    SomsiadResult,
    SomsiadStatus
};
use crate::guards::{
    maintenance_mode::*,
    static_files::*
};

#[catch(default)]
pub fn default_catcher(status: Status, _req: &Request) -> (Status, SomsiadResult<&'static str>) {
    (status, SomsiadStatus::error("Nieoczekiwany błąd"))
}

#[catch(503)]
pub async fn maintenance_catch() 
-> Result<(Status, NamedFile), (Status, &'static str)> {
    match NamedFile::open(Path::new(relative!(""))
        .join("maintenance.html"))
        .await {
            Ok(f) => Ok((
                Status::ServiceUnavailable,
                f
            )),
            Err(_) => Err((
                Status::ServiceUnavailable,
                "Strona jest w trakcie prac konserwacyjnych. Przepraszamy za utrudnienia!"
            ))
        }
}

#[catch(404)]
pub async fn spa_catcher() -> Result<(Status, NamedFile), (Status, SomsiadResult<&'static str>)> {
    let path = Path::new(relative!("static"))
        .join("index.html");

    let file = NamedFile::open(path).await;

    match file {
        Ok(f) => Ok((
            Status::Ok,
            f
        )),
        Err(_) => Err((
            Status::NotFound,
            SomsiadStatus::error("Wystąpił błąd przy wczytywaniu strony. Spróbuj ponownie. Jeśli problem się powtórzy, skontaktuj się z administratorem")
        ))
    }
}

#[get("/<_path..>", rank = 1)]
pub async fn get_page(_path: PathBuf,_maintenance: MaintenanceMode, _static: StaticFiles) -> (Status, SomsiadResult<&'static str>) {
    (
        Status::InternalServerError,
        SomsiadStatus::error("Nieoczekiwany błąd")
    )
}
