#![allow(dead_code)]

mod types;
mod bitboard;
mod position;
mod perft;

use perft::perft;

fn main() {
    perft();
}