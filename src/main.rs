#[allow(unused_imports)]
use i18n_embed::{
    fluent::{fluent_language_loader, FluentLanguageLoader},
    select, LanguageLoader,
};
#[allow(unused_imports)]
use i18n_embed_fl::fl;
#[allow(unused_imports)]
use rocket::{
    catchers,
    fs::{relative, FileServer},
    launch,
    serde::json::Json,
    Build, Rocket, State,
};
#[allow(unused_imports)]
use rust_embed::RustEmbed;

pub mod about;
pub mod locales;
pub mod not_found;

#[allow(unused_imports)]
use self::locales::{I18nHelper, Localizations};

#[cfg(not(test))]
#[launch]
pub fn rocket() -> Rocket<Build> {
    // Initialize the loader
    let loader: FluentLanguageLoader = fluent_language_loader!();

    // Load all languages from the embedded assets
    loader
        .load_languages(&Localizations, &[loader.fallback_language().clone()])
        .expect("Failed to load languages");

    rocket::build()
        .manage(I18nHelper { loader })
        .mount(
            "/",
            FileServer::new(relative!("rsrc/io.favicon/emoji/Zzz/")),
        )
        .mount("/", rocket::routes![about::get_about])
        .register("/", catchers![not_found::not_found])
}
