#![allow(dead_code)]
#![allow(unused_imports)]

mod types;
mod bitboard;
mod position;
mod perft;

use lazy_static::lazy_static;

use perft::perft;

fn main() {
    perft();
}