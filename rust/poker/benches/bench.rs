use criterion::{criterion_group, criterion_main, Criterion};

use poker::test;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("tie has multiple winners", |b| {
        b.iter(|| {
            test(
                &[
                    "4D 5S 6S 8D 3C",
                    "2S 4C 7S 9H 10H",
                    "3S 4S 5D 6H JH",
                    "3H 4H 5C 6C JD",
                ],
                &["3S 4S 5D 6H JH", "3H 4H 5C 6C JD"],
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
