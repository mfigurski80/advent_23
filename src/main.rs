#![feature(iter_advance_by)]
#![feature(iter_array_chunks)]
#![allow(dead_code)]
extern crate regex;

mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
pub mod io_utils;
pub mod range_utils;

fn main() {
    d5::run();
}
