use rocket::{
    http::{Status, ContentType},
    get,
    catch,
    form::FromForm,
    Request
};
use reqwest;
use crate::{
    SomsiadResult,
    SomsiadStatus,
    guards::maintenance_mode::*
};

#[derive(FromForm, Debug)]
pub struct Positionstack<'r> {
    pub access_key: &'r str,
    pub query: &'r str,
    pub country: Option<&'r str>,
    pub region: Option<&'r str>,
    pub language: Option<&'r str>,
    pub country_module: Option<usize>,
    pub sun_module: Option<usize>,
    pub timezone_module: Option<usize>,
    pub bbox_module: Option<usize>,
    pub limit: Option<usize>,
    pub fields: Option<&'r str>,
    pub callback: Option<&'r str>
}

#[catch(default)]
pub fn default_catcher(status: Status, _req: &Request) -> (Status, SomsiadResult<&'static str>) {
    (
        status,
        SomsiadStatus::error("Nieoczekiwany błąd")
    )
} 

#[catch(503)]
pub fn maintenance_catcher() -> SomsiadResult<&'static str> {
    SomsiadStatus::error("Serwis jest w trakcie prac konserwacyjnych")
}

#[get("/<op_type>?<query..>")]
pub async fn get_positionstack(
    op_type: String,
    query: Positionstack<'_>,
    _maintenance: MaintenanceMode
) -> Result<(Status, (ContentType, String)), (Status, SomsiadResult<&'static str>)> {
    if op_type.eq("forward") || op_type.eq("reverse") {
        let mut uri = format!(
            "http://api.positionstack.com/v1/{}?access_key={}&query={}", 
            op_type,
            query.access_key,
            query.query
        );

        if let Some(a) = query.country {
            uri.push_str(
                format!(
                    "&country={}",
                    a
                ).as_str()
            )
        }
        if let Some(a) = query.region {
            uri.push_str(
                format!(
                    "&region={}",
                    a
                ).as_str()
            )
        }
        if let Some(a) = query.language {
            uri.push_str(
                format!(
                    "&language={}",
                    a
                ).as_str()
            )
        }
        if let Some(a) = query.country_module {
            uri.push_str(
                format!(
                    "&country_module={}",
                    a
                ).as_str()
            )
        }
        if let Some(a) = query.sun_module {
            uri.push_str(
                format!(
                    "&sun_module={}",
                    a
                ).as_str()
            )
        }
        if let Some(a) = query.timezone_module {
            uri.push_str(
                format!(
                    "&timezone_module={}",
                    a
                ).as_str()
            )
        }
        if let Some(a) = query.bbox_module {
            uri.push_str(
                format!(
                    "&bbox_module={}",
                    a
                ).as_str()
            )
        }
        if let Some(a) = query.limit {
            uri.push_str(
                format!(
                    "&limit={}",
                    a
                ).as_str()
            )
        }
        if let Some(a) = query.fields {
            uri.push_str(
                format!(
                    "&fields={}",
                    a
                ).as_str()
            )
        }
        if let Some(a) = query.callback {
            uri.push_str(
                format!(
                    "&callback={}",
                    a
                ).as_str()
            )
        }

        match reqwest::get(uri.as_str())
            .await {
            Ok(r) => {
                let code = r
                    .status()
                    .as_u16();

                match r.text()
                    .await {
                    Ok(t) => Ok((
                        Status::from_code(code).unwrap_or_else(|| Status::Ok),
                        (
                            ContentType::JSON,
                            t
                        )
                    )),
                    Err(_) => Err((
                        Status::InternalServerError,
                        SomsiadStatus::error("Nieoczekiwany błąd")
                    )) 
                }
            },
            Err(_) => Err((
                Status::InternalServerError,
                SomsiadStatus::error("Nieoczekiwany błąd")
            ))
        }
    } else {
        Err((
            Status::BadRequest,
            SomsiadStatus::error(
                "Zły typ zapytania. Oczekiwano \"forward\" lub \"reverse\""
            )
        ))
    }
}
