#[macro_use]
extern crate criterion;

extern crate rust_experiments;

use criterion::Criterion;

use rust_experiments::cities::*;

fn gen_uset(c: &mut Criterion) {
    c.bench_function("USet generate map 1000", |b| {
        b.iter({ || gen_cities_uset(1000, 50) })
    });
}

fn gen_hashset(c: &mut Criterion) {
    c.bench_function("HashSet generate map 1000", |b| {
        b.iter({ || gen_cities_hashset(1000, 50) })
    });
}

fn gen_imset(c: &mut Criterion) {
    c.bench_function("ImSet generate map 1000", |b| {
        b.iter({ || gen_cities_imset(1000, 50) })
    });
}

fn gen_standard(c: &mut Criterion) {
    c.bench_function("Standard generate map 1000", |b| {
        b.iter({ || gen_cities(1000, 5, 0.2, 15) })
    });
}

fn solve_uset(c: &mut Criterion) {
    let map = gen_cities_uset(5000, 75);
    c.bench_function("Solve map 5000", |b| {
        b.iter({ || find_city_distances(&map) })
    });
}

criterion_group!(benches, gen_uset, gen_hashset, gen_imset, gen_standard, solve_uset);
criterion_main!(benches);
