use criterion::{criterion_group, criterion_main, Criterion};

use std::str::FromStr;

fn criterion_benchmark(c: &mut Criterion) {
    let edn = edn_str();
    c.bench_function("parse", |b| b.iter(|| edn_rs::Edn::from_str(&edn)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn edn_str() -> String {
    "{
        :type :human
        :first-name \"bench\"
        :last-name \"mark\"
        :age 13
        :version 0.13
        :associates [
            {
                :name :julia
                :role :adm
            }
            {
                :name :otavio
                :role :contributor
            }
            {
                :name :juxt
                :role :great-ideas
            }
        ]
    }"
    .to_string()
}
