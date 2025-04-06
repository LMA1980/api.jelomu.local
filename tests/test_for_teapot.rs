#[allow(unused_imports)] use rocket::{
    launch, routes, catchers, uri, Build, Rocket, 
    http::{ ContentType, ext, hyper, uncased, uri, Status}, 
    local::asynchronous::Client,
};
#[allow(unused_imports)] use rocket_include_static_resources::{
    static_resources_initializer, static_response_handler
};
#[warn(unused_imports)]
#[path = "../src/main.rs"] mod webapi;
use crate::webapi::{about::{About}};

#[rocket::tokio::test]  // Use tokio for asynchronous tests (reqwest is async)
pub async fn test_request_get_about_data() {
    let client: Client = Client::untracked(rocket::build()
        .mount("/", rocket::routes![webapi::about::get_about])).await.unwrap();
    let response = client.get("/about")
        .dispatch().await;
    let about_object: About = response.into_json::<About>()
        .await.expect("valid json response");
    assert_eq!(about_object, About
    {
        version: "2025.Q1".to_string(),
        features: vec![
            "Common".to_string(),
        ],
        status: "Yes! You did find the Teapot!".to_string(),
    });
}
#[rocket::tokio::test]  // Use tokio for asynchronous tests (reqwest is async)
pub async fn test_request_get_about_httpcode_im_a_teapot() {
    let client: Client = Client::untracked(rocket::build()
        .mount("/", rocket::routes![webapi::about::get_about])).await.unwrap();
    let response = client.get("/about")
        .dispatch().await;
    assert_eq!(response.status(), Status::ImATeapot);
}

#[rocket::tokio::test]  // Use tokio for asynchronous tests (reqwest is async)
pub async fn test_request_get_about_contenttype_json() {
    let client: Client = Client::untracked(rocket::build()
        .mount("/", rocket::routes![webapi::about::get_about])).await.unwrap();
    let response = client.get("/about")
        .dispatch().await;
    assert_eq!(response.content_type(), Some(ContentType::JSON));
}
