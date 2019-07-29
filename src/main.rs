#![allow(unknown_lints)]

use std::env;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[cfg(test)]
#[macro_use]
extern crate spectral;

extern crate flexi_logger;
#[macro_use]
extern crate log;

extern crate itertools;

pub mod cities;
#[macro_use]
pub mod utils;

pub mod arena;

pub mod strings;

pub mod redis;

extern crate rand;

use flexi_logger::*;

fn main() {
    let mut b = LogSpecBuilder::new();
    b.default(log::LevelFilter::Info);
    let spec = b.finalize();
    Logger::with(spec)
        .log_to_file()
        .directory("log")
        .print_message()
        .format(flexi_logger::detailed_format)
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));

    //redis::main();
    cities();
    //strings::strings();
}

fn cities() {
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

    info!("Cities standard generator chosen with options: size={}, max_broad={}, knots_occurence={}, knots_max_broad={}",
          size, max_broad, knots_occurence, knots_max_broad
    );

    let city_array = cities::gen_cities(size, max_broad, knots_occurence, knots_max_broad);

    info!("Map generated");

    let result = cities::find_city_distances(&city_array);

    info!("Map solved: {:?}", result);
}
