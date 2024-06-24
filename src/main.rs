#![allow(dead_code)]
#![allow(unused_imports)]

mod types;
mod bitboard;
mod position;
mod game;

use lazy_static::lazy_static;

use crate::types::*;
use crate::bitboard::*;
use crate::position::Position;

fn main() {
    let mut pos = Position::new();
    pos.parse_fen("8/4K3/8/8/1R1pP1k1/8/8/8 b - e3 0 1");
    println!("{}", pos);

    let mut move_list = [Move::default(); 256];
    let mut move_count: usize = 0;
    pos.gen_legal_moves(&mut move_list, &mut move_count);
    
    println!("moves: {}", move_count);
    for i in 0..move_count {
        let m = move_list[i];
        println!("{}: {}", PIECE_NAMES[pos.piece_at(m.from_square) as usize], m);
        
    }
}