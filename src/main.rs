#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;
#[macro_use] extern crate serde_json;

use rocket::{Build, Rocket};
mod about;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![about::get_about])
}