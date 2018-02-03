#![feature(test)]
extern crate rust_experiments;
extern crate test;

use rust_experiments::cities::*;

#[cfg(test)]
mod benchmark_tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn generate_standard_1000(b: &mut Bencher) {
        b.iter(|| {
            gen_cities(1000, 5, 0.1, 25);
        });
    }

    #[bench]
    fn generate_standard_10000(b: &mut Bencher) {
        b.iter(|| {
            gen_cities(10_000, 5, 0.1, 25);
        });
    }

    #[bench]
    fn generate_standard_25000(b: &mut Bencher) {
        b.iter(|| {
            gen_cities(25_000, 5, 0.1, 25);
        });
    }

    #[bench]
    fn solve_standard_1000(b: &mut Bencher) {
        let map = gen_cities(1000, 5, 0.1, 25);
        b.iter(|| {
            find_city_distances(&map);
        });
    }

    #[bench]
    fn solve_standard_10000(b: &mut Bencher) {
        let map = gen_cities(10_000, 5, 0.1, 25);
        b.iter(|| {
            find_city_distances(&map);
        });
    }

    #[bench]
    fn solve_standard_25000(b: &mut Bencher) {
        let map = gen_cities(25_000, 5, 0.1, 25);
        b.iter(|| {
            find_city_distances(&map);
        });
    }

    #[bench]
    fn generate_uset_1000(b: &mut Bencher) {
        b.iter(|| {
            gen_cities_uset(1000, 50);
        });
    }

    #[bench]
    fn generate_uset_10000(b: &mut Bencher) {
        b.iter(|| {
            gen_cities_uset(10_000, 100);
        });
    }

    #[bench]
    fn generate_uset_25000(b: &mut Bencher) {
        b.iter(|| {
            gen_cities_uset(25_000, 125);
        });
    }

    #[bench]
    fn solve_uset_1000(b: &mut Bencher) {
        let map = gen_cities_uset(1000, 50);
        b.iter(|| {
            find_city_distances(&map);
        });
    }

    #[bench]
    fn solve_uset_10000(b: &mut Bencher) {
        let map = gen_cities_uset(10_000, 100);
        b.iter(|| {
            find_city_distances(&map);
        });
    }

    #[bench]
    fn solve_uset_25000(b: &mut Bencher) {
        let map = gen_cities_uset(25_000, 125);
        b.iter(|| {
            find_city_distances(&map);
        });
    }

}
