use i18n_embed::fluent::FluentLanguageLoader;
#[allow(unused_imports)]
use rust_embed::RustEmbed;

#[allow(unused)]
#[derive(RustEmbed)]
#[folder = "rsrc/i18n/"]
pub struct Localizations;

pub struct I18n {
    pub loader: FluentLanguageLoader,
}
