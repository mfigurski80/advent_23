#![feature(iter_advance_by)]
#![feature(iter_array_chunks)]
#![feature(slice_group_by)]
#![feature(coroutines)]
#![feature(coroutine_trait)]
#![allow(dead_code)]
extern crate ndarray;
extern crate regex;

mod d1;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d15;
mod d16;
mod d2;
mod d3;
mod d4;
mod d5;
mod d7;
mod d8;
mod d9;
pub mod graph;
pub mod io_utils;
pub mod map_utils;
pub mod range_utils;

fn main() {
    d16::run();
}
