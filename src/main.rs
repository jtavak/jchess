#![allow(dead_code)]
#![allow(unused_imports)]

mod types;
mod bitboard;
mod position;

use lazy_static::lazy_static;

use crate::types::*;
use crate::bitboard::*;
use crate::position::Position;

fn count_nodes(pos: &Position, depth: usize) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut move_list: [Move; 256] = [Move::default(); 256];
    let mut move_count: usize = 0;

    pos.gen_legal_moves(&mut move_list, &mut move_count);

    let mut total_count: u64 = 0;
    for i in 0..move_count {
        let mut updated_pos = *pos;
        updated_pos.make(&move_list[i]);

        let nodes: u64 = count_nodes(&updated_pos, depth-1);

        if depth == 4 {
            println!("{}{}: {}", SQUARE_NAMES[move_list[i].from_square], SQUARE_NAMES[move_list[i].to_square],  nodes);
        }

        total_count += nodes;
    }

    return total_count;
}

fn main() {
    let mut pos = Position::new();
    pos.parse_fen("8/2p5/3p4/KP3k1r/6R1/8/4P1P1/8 w - - 3 3");


    println!("{}", pos);

    println!("Nodes: {}", count_nodes(&pos, 4))
}