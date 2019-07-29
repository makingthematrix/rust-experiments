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
