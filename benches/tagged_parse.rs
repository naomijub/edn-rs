use criterion::{criterion_group, criterion_main, Criterion};
use edn_rs;
use std::str::FromStr;

fn criterion_benchmark(c: &mut Criterion) {
    let edn = edn_str();
    c.bench_function("tagged_parse", |b| b.iter(|| edn_rs::Edn::from_str(&edn)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn edn_str() -> String {
    "{
        :time #inst \"2020-10-12T09:30:00-00:00\"
        :name \"benchmark rust code\"
        :id #uuid \"af6d8699-f442-4dfd-8b26-37d80543186b\"
    }"
    .to_string()
}
