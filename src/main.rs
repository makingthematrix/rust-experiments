use std::env;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[cfg(test)]
#[macro_use]
extern crate spectral;

extern crate im;

pub mod cities;
pub mod utils;

fn main() {
    let size = match env::args().nth(1) {
        Some(s) => s.parse::<usize>().unwrap(),
        None => 25_000,
    };

    let max_broad = match env::args().nth(2) {
        Some(max) => max.parse::<usize>().unwrap(),
        None => 5,
    };

    let knots_occurence = match env::args().nth(3) {
        Some(occ) => occ.parse::<f32>().unwrap(),
        None => 0.1,
    };

    let knots_max_broad = match env::args().nth(4) {
        Some(max) => max.parse::<usize>().unwrap(),
        None => 25,
    };

    let city_array = cities::gen_cities(size, max_broad, knots_occurence, knots_max_broad);
    let result = cities::find_city_distances(&city_array);

    println!("{:?}", result);
}
