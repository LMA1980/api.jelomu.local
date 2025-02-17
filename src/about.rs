use rocket::{get, {http::Status}, {serde::json::Json}};
#[derive(serde::Deserialize, serde::Serialize, PartialEq, Clone, Debug)]
pub struct About {
    pub version: String,
    pub features: Vec<String>,
    pub status: String,
}
impl About {
    #[allow(dead_code)]
    pub fn new(version: String, features: Vec<String>, status: String) -> Self {
        Self { version, features, status }
    }
}
#[get("/about")]
pub fn get_about() -> (Status, Json<About>)
{
    (
        Status::ImATeapot,
         Json(About
         {
            version: "2025.Q1".to_string(),
            features: vec![
                "Common".to_string(),
            ],
            status: "Yes! You did find the Teapot!".to_string(),
        })
    )
}

#[cfg(test)]
use rocket::tokio::test;
#[test]
async fn test_get_about() {
    let (status, json_about): (Status, Json<About>) = get_about();
    assert_eq!(status, rocket::http::Status::ImATeapot);
    let about_object: About = json_about.into_inner();
    assert_eq!(about_object, About
    {
        version: "2025.Q1".to_string(),
        features: vec![
            "Common".to_string(),
        ],
        status: "Yes! You did find the Teapot!".to_string(),
    });
}