#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
extern crate lv2_raw;

mod bindgen;

pub use bindgen::*;

#[link(name = "suil-0")]
extern "C" {}
