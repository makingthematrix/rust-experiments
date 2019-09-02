#[macro_use]
extern crate criterion;

extern crate rust_experiments;

use criterion::Criterion;

use rust_experiments::cities::*;

fn gen_uset(c: &mut Criterion) {
    c.bench_function("USet generate map 1000", |b| {
        b.iter({ || gen_cities_uset(1000, 75) })
    });
}

fn gen_hashset(c: &mut Criterion) {
    c.bench_function("HashSet generate map 1000", |b| {
        b.iter({ || gen_cities_hashset(1000, 75) })
    });
}

fn solve(c: &mut Criterion) {
    let map = gen_cities_uset(1000, 75);
    c.bench_function("Solve map 1000", move |b| {
        b.iter({ || find_city_distances(&map) })
    });
}

criterion_group!(benches, gen_uset, gen_hashset, solve);
criterion_main!(benches);
