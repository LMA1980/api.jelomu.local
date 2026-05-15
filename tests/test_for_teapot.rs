use i18n_embed::LanguageLoader;
use jeroapi::{self, locales::I18n};
#[allow(unused_imports)]
#[macro_use]
extern crate rocket;
#[allow(unused_imports)]
use rocket::{
    catchers,
    http::{ext, uncased, uri, ContentType, Header, Status},
    launch,
    local::asynchronous::Client,
    routes, uri, Build, Rocket, State,
};
#[rocket::tokio::test] // Use tokio for asynchronous tests (reqwest is async)
pub async fn test_request_get_about_data() {
    let loader: i18n_embed::fluent::FluentLanguageLoader =
        i18n_embed::fluent::fluent_language_loader!();
    loader
        .load_languages(
            &jeroapi::locales::Localizations,
            &[loader.fallback_language().clone()],
        )
        .expect("Failed to load languages");
    let client: Client = Client::untracked(
        rocket::build()
            .manage(I18n { loader })
            .mount("/", rocket::routes![jeroapi::about::rocket::get_about]),
    )
    .await
    .unwrap();
    let response = client
        .get("/about")
        .header(Header::new("Accept-Language", "en-US"))
        .dispatch()
        .await;
    let about_object: jeroapi::about::About = response
        .into_json::<jeroapi::about::About>()
        .await
        .expect("valid json response");
    assert_eq!(
        about_object,
        jeroapi::about::About {
            version: "2026.Q1".to_string(),
            features: vec!["Common".to_string(),],
            status: "Yes! You did find the Teapot!".to_string(),
        }
    );
}
#[rocket::tokio::test] // Use tokio for asynchronous tests (reqwest is async)
pub async fn test_request_get_about_httpcode_im_a_teapot() {
    let loader: i18n_embed::fluent::FluentLanguageLoader =
        i18n_embed::fluent::fluent_language_loader!();
    loader
        .load_languages(
            &jeroapi::locales::Localizations,
            &[loader.fallback_language().clone()],
        )
        .expect("Failed to load languages");
    let client: Client = Client::untracked(
        rocket::build()
            .manage(I18n { loader })
            .mount("/", rocket::routes![jeroapi::about::rocket::get_about]),
    )
    .await
    .unwrap();
    let response = client.get("/about").dispatch().await;
    assert_eq!(response.status(), Status::ImATeapot);
}

#[rocket::tokio::test] // Use tokio for asynchronous tests (reqwest is async)
pub async fn test_request_get_about_contenttype_json() {
    let loader: i18n_embed::fluent::FluentLanguageLoader =
        i18n_embed::fluent::fluent_language_loader!();
    loader
        .load_languages(
            &jeroapi::locales::Localizations,
            &[loader.fallback_language().clone()],
        )
        .expect("Failed to load languages");
    let client: Client = Client::untracked(
        rocket::build()
            .manage(I18n { loader })
            .mount("/", rocket::routes![jeroapi::about::rocket::get_about]),
    )
    .await
    .unwrap();
    let response = client.get("/about").dispatch().await;
    assert_eq!(response.content_type(), Some(ContentType::JSON));
}
