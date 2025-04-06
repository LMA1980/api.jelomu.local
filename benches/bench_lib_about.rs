#[allow(unused_imports)] #[cfg(test)] #[macro_use] extern crate criterion;
#[cfg(test)] use criterion::{black_box, Criterion, criterion_group, criterion_main};
#[path = "../src/main.rs"] mod webapi;
#[allow(unused_imports)] use crate::webapi::about;

fn bench_test_about(c: &mut Criterion) {
    c.bench_function(
        "about",
        |f| f.iter(|| {
            black_box(webapi::about::get_about())
        }));
}

const SAMPLE_BASE:u16 = 255;
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
