use crate::locales::I18n;
#[cfg(test)]
use crate::locales::Localizations;

#[cfg(test)]
use i18n_embed::LanguageLoader;
use i18n_embed_fl::fl;

#[derive(serde::Deserialize, serde::Serialize, PartialEq, Clone, Debug)]
pub struct About {
    pub version: String,
    pub features: Vec<String>,
    pub status: String,
}
impl About {
    pub fn localized(i18n: &I18n) -> Self {
        Self {
            version: fl!(i18n.loader, "version"),
            features: vec![fl!(i18n.loader, "features_common")],
            status: fl!(i18n.loader, "about_status"),
        }
    }
}

pub mod rocket {
    use super::About;
    use crate::locales::I18n;
    use rocket::{self, get, http::Status, serde::json::Json, State};
    #[get("/about")]
    pub fn get_about(i18n: &State<I18n>) -> (Status, Json<About>) {
        (
            //Status::Ok,
            Status::ImATeapot,
            Json(About::localized(i18n.inner())),
        )
    }
}
//------ UnitTest --------------------------------------------------------------------------------
#[cfg(test)]
#[test]
fn test_localized() {
    // Initialize the loader
    let loader: i18n_embed::fluent::FluentLanguageLoader =
        i18n_embed::fluent::fluent_language_loader!();

    // Load all languages from the embedded assets
    loader
        .load_languages(&Localizations, &[loader.fallback_language().clone()])
        .expect("Failed to load languages");

    let i18n = I18n { loader };
    let about = About::localized(&i18n);
    assert_eq!(about.status, "Yes! You did find the Teapot!");
}
