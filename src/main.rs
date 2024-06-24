#![allow(dead_code)]
#![allow(unused_imports)]

mod types;
mod bitboard;
mod position;

use lazy_static::lazy_static;

use crate::types::*;
use crate::bitboard::*;
use crate::position::Position;

fn count_nodes(states: &mut [Position; 32], depth: usize, max_depth: usize) -> u32 {
    if depth == max_depth {
        return 1;
    }

    let mut move_list: [Move; 256] = [Move::default(); 256];
    let mut move_count: usize = 0;

    states[depth].gen_legal_moves(&mut move_list, &mut move_count);

    let mut total_count: u32 = 0;
    for i in 0..move_count {
        states[depth+1] = states[depth];
        states[depth+1].make(&move_list[i]);

        let nodes: u32 = count_nodes(states, depth+1, max_depth);

        if depth == 0 {
            println!("{}: {}", move_list[i], nodes);
        }

        total_count += nodes;
    }

    return total_count;
}

fn main() {
    let mut states: [Position; 32] = [Position::default(); 32];

    let mut pos = Position::new();
    
    // TODO: figure out why this perft differs from stockfish at depth 5
    pos.parse_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ");

    states[0] = pos;

    println!("{}", pos);

    println!("Nodes: {}", count_nodes(&mut states, 0, 5))
}