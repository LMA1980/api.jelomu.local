// #[cfg(not(test))]
// #[allow(unused_imports)]
// #[macro_use] extern crate rocket;
// #[cfg(not(test))]
// #[allow(unused_imports)]
// #[macro_use] extern crate serde;
// #[cfg(not(test))]
// #[allow(unused_imports)]
// #[macro_use] extern crate serde_json;
#[allow(unused_imports)]
use rocket::{
    catchers,
    fs::{relative, FileServer},
    launch,
    serde::json::Json,
    Build, Rocket,
};

pub mod about;
pub mod not_found;

#[cfg(not(test))]
#[launch]
pub fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount(
            "/",
            FileServer::new(relative!("rsrc/io.favicon/emoji/Zzz/")),
        )
        .mount("/", rocket::routes![about::get_about])
        .register("/", catchers![not_found::not_found])
}
