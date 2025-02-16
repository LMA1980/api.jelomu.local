use rocket::http::Status;
use rocket::serde::json::Json;
#[derive(rocket::serde::Deserialize, serde::Serialize, Debug)]
struct About {
    version: String,
    features: Vec<String>,
    status: String,
}
impl About {
    pub fn new(version: String, features: Vec<String>, status: String) -> Self {
        Self { version, features, status }
    }
}
#[get("/about")]
pub async fn get_about() -> (Status, Json<About>)
{
    (
        Status::ImATeapot,
         Json(About
         {
            version: "2025.Q1".to_string(),
            features: vec![
                "Default".to_string(),
            ],
            status: "Yes! You did find the Teapot!".to_string(),
        })
    )
}