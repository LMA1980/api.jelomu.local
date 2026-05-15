#[macro_use]
extern crate rocket;
use rocket::{
    fs::{relative, FileServer},
    Build, Rocket,
};

#[allow(unused_imports)]
use i18n_embed::{
    fluent::{fluent_language_loader, FluentLanguageLoader},
    select, LanguageLoader,
};
#[allow(unused_imports)]
use i18n_embed_fl::fl;
//use rocket::{
//    self, catchers,
//    fs::{relative, FileServer},
//    launch,
//    serde::json::Json,
//    Build, Rocket, State,
//};
#[allow(unused_imports)]
use rust_embed::RustEmbed;

#[allow(unused_imports)]
use jeroapi::{
    self,
    about::rocket::get_about,
    locales::{I18n, Localizations},
    not_found::not_found,
};

#[launch]
pub fn rocket() -> Rocket<Build> {
    // Initialize the loader
    let loader: FluentLanguageLoader = fluent_language_loader!();

    // Load all languages from the embedded assets
    loader
        .load_languages(&Localizations, &[loader.fallback_language().clone()])
        .expect("Failed to load languages");

    rocket::build()
        .manage(I18n { loader })
        .mount(
            "/",
            FileServer::new(relative!("rsrc/io.favicon/emoji/Zzz/")),
        )
        .mount("/", rocket::routes![get_about])
        .register("/", catchers![not_found])
}
