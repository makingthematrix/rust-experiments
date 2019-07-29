extern crate rust_experiments;

use rust_experiments::cities::*;

#[test]
fn generate_and_solve_standard() {
    let map = gen_cities(1000, 5, 0.1, 25);
    let result = find_city_distances(&map);
    println!("{:?}", result);
}

#[test]
fn generate_and_solve_uset() {
    let map = gen_cities_uset(5, 25);
    let result = find_city_distances(&map);
    println!("{:?}", result);
}

#[test]
fn generate_and_solve_hashset() {
    let map = gen_cities_hashset(1000, 25);
    let result = find_city_distances(&map);
    println!("{:?}", result);
}
