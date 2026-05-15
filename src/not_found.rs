use super::locales::I18n;
use i18n_embed_fl::fl;
use rocket::http::{ContentType, Status};
use rocket::{catch, serde::json::Json, Request};
use serde::Serialize;

#[allow(unused)]
#[derive(Serialize)]
pub struct ProblemDetails {
    #[serde(rename = "type")]
    pub _type: String,
    pub title: String,
    pub status: u16,
    pub detail: String,
    pub instance: String,
}

#[allow(unused)]
#[catch(404)]
pub fn not_found(status: Status, req: &Request) -> (Status, (ContentType, Json<ProblemDetails>)) {
    // 1. Get the path the user actually typed
    let path = req.uri().path().as_str();

    // 2. Build your documentation link (Optional: replace with your actual domain)
    let error_type = format!("https://api.localhost.local/errors/not-found?path={}", path);
    let i18n: &I18n = req
        .rocket()
        .state::<I18n>()
        .expect("I18n not managed in Rocket");

    let problem = ProblemDetails {
        _type: error_type,
        title: "Endpoint Not Found".to_string(),
        status: 404,
        detail: fl!(i18n.loader, "not_found_detail", path = path),
        instance: path.to_string(), // The specific URI that caused the error
    };

    // We return a 3-part tuple: Status, ContentType, and the JSON body
    (
        Status::NotFound,
        (
            ContentType::new("application", "problem+json"),
            Json(problem),
        ),
    )
}
