#![allow(dead_code)]

mod types;
mod bitboard;
mod position;
mod movegen;
mod perft;

use std::{collections::VecDeque, sync::{Arc, Mutex}, thread};

use position::Position;
use types::*;
use movegen::gen_legal_moves;

fn fast_perft(pos: Position, depth: u8, thread_count: u8) -> u64 {
    let mut move_list: [Move; 256] = [Move::default(); 256];
    let mut move_count = 0;

    gen_legal_moves(&pos, &mut move_list, &mut move_count);

    let queue: Arc<Mutex<VecDeque<Move>>> = Arc::new(Mutex::new(move_list.iter().take(move_count).copied().collect()));

    let mut handles = vec![];

    for _ in 0..thread_count {
        let queue: Arc<Mutex<VecDeque<Move>>> = Arc::clone(&queue);
        let handle = thread::spawn(move || {
            let mut thread_sum: u64 = 0;

            loop {
                let move_p = {
                    let mut queue = queue.lock().unwrap();
                    queue.pop_front()
                };

                if let Some(mv) = move_p {
                    let mut updated_pos = pos;
                    updated_pos.make(&mv);

                    thread_sum += perft::perft(&updated_pos, depth-1);
                } else {
                    break;
                }
            }


            thread_sum
        });

        handles.push(handle);
    }

    let mut perft_result = 0;
    for handle in handles{
        perft_result += handle.join().unwrap();
    }

    perft_result
}

fn main() {
    let p = Position::new();
    println!("{}", fast_perft(p, 6, 8));
}