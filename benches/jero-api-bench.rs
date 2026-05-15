#[allow(unused_imports)]
#[cfg(test)]
#[macro_use]
extern crate criterion;
#[cfg(test)]
use criterion::{criterion_group, criterion_main, Criterion};
#[cfg(test)]
use i18n_embed::LanguageLoader;
#[cfg(test)]
use jeroapi::locales::{I18n, Localizations};

fn bench_test_about(c: &mut Criterion) {
    // Initialize the loader
    let loader: i18n_embed::fluent::FluentLanguageLoader =
        i18n_embed::fluent::fluent_language_loader!();

    // Load all languages from the embedded assets
    loader
        .load_languages(&Localizations, &[loader.fallback_language().clone()])
        .expect("Failed to load languages");

    let i18n = I18n { loader };
    c.bench_function("about", |f| {
        f.iter(|| std::hint::black_box(jeroapi::about::About::localized(&i18n)))
    });
}

const SAMPLE_BASE: u16 = 255;
const SAMPLE_TIME_FACTOR: u16 = 1;

criterion_group! {
    name = benches;
    config = Criterion::default()
        .sample_size(
            (SAMPLE_BASE) as usize * 10
        )
        .measurement_time(
            std::time::Duration::from_secs(
            (SAMPLE_BASE * SAMPLE_TIME_FACTOR) as u64
        ));
    targets = bench_test_about,
}
criterion_main!(benches);
