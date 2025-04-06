#[allow(unused_imports)] #[macro_use] extern crate criterion;
#[cfg(test)] use criterion::*;
#[allow(unused_imports)] use criterion::black_box;
#[allow(unused_imports)] #[cfg(test)] use criterion::async_executor::FuturesExecutor;
#[path = "../src/main.rs"] mod webapi;
#[path = "../tests/test_for_teapot.rs"] mod tests;

fn bench_test_for_teapot(c: &mut Criterion) {
    c.bench_function(
        "test_for_teapot",
        move |bencher: &mut Bencher|
            bencher.to_async(
                criterion::async_executor::FuturesExecutor).iter(
                    || async {
                        tests::test_request_get_about_httpcode_im_a_teapot();
                    }
            ));
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
    targets = bench_test_for_teapot,
}
criterion_main!(benches);
