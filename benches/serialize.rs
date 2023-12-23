use criterion::{criterion_group, criterion_main};

criterion_group!(
    benches,
    serde::criterion_benchmark,
    edn::criterion_benchmark
);
criterion_main!(benches);

mod serde {
    use criterion::Criterion;
    use edn_rs::{map, set};
    use serde::Serialize;
    use std::collections::{BTreeMap, BTreeSet};

    pub fn criterion_benchmark(c: &mut Criterion) {
        c.bench_function("serde", |b| {
            b.iter(|| serde_json::to_string(&val()).unwrap());
        });
    }

    fn val() -> Val {
        Val {
            btreemap: map! {"this is a key".to_string() => vec!["with".to_string(), "many".to_string(), "keys".to_string()]},
            btreeset: set! {3i64, 4i64, 5i64},
            tuples: (3i32, true, 'd'),
            foo_vec: vec![Foo { value: 2 }, Foo { value: 3 }],
        }
    }

    #[derive(Debug, Clone, Serialize)]
    struct Val {
        btreemap: BTreeMap<String, Vec<String>>,
        btreeset: BTreeSet<i64>,
        tuples: (i32, bool, char),
        foo_vec: Vec<Foo>,
    }

    #[derive(Debug, Clone, Serialize)]
    struct Foo {
        value: u64,
    }
}

mod edn {
    use criterion::Criterion;
    use edn_derive::Serialize;

    use edn_rs::{map, set, Serialize};
    use std::collections::{BTreeMap, BTreeSet};

    pub fn criterion_benchmark(c: &mut Criterion) {
        let val = val();
        c.bench_function("edn", |b| b.iter(|| val.serialize()));
    }

    fn val() -> ValEdn {
        ValEdn {
            btreemap: map! {"this is a key".to_string() => vec!["with".to_string(), "many".to_string(), "keys".to_string()]},
            btreeset: set! {3i64, 4i64, 5i64},
            tuples: (3i32, true, 'd'),
            foo_vec: vec![Foo { value: 2 }, Foo { value: 3 }],
        }
    }

    #[derive(Debug, Clone, Serialize)]
    struct ValEdn {
        btreemap: BTreeMap<String, Vec<String>>,
        btreeset: BTreeSet<i64>,
        tuples: (i32, bool, char),
        foo_vec: Vec<Foo>,
    }

    #[derive(Debug, Clone, Serialize)]
    struct Foo {
        value: u64,
    }
}
