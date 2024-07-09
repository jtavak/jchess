use crate::types::*;
use crate::movegen::gen_legal_moves;
use crate::position::Position;

// check move generation against positions from https://www.chessprogramming.org/Perft_Results

struct PerftResult {
    fen: &'static str,
    depth: u8,
    move_count: u64
}

const PERFT_RESULTS: [PerftResult; 6] = [
    PerftResult {fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", depth: 6, move_count: 119_060_324},
    PerftResult {fen: "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -", depth: 5, move_count: 193_690_690},
    PerftResult {fen: "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ", depth: 7, move_count: 178_633_661},
    PerftResult {fen: "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", depth: 6, move_count: 706_045_033},
    PerftResult {fen: "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8", depth: 5, move_count: 89_941_194},
    PerftResult {fen: "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10", depth: 5, move_count: 164_075_551}
];

pub fn perft(pos: &Position, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut move_list: [Move; 256] = [Move::default(); 256];
    let mut move_count: usize = 0;

    gen_legal_moves(pos, &mut move_list, &mut move_count);

    let mut total_count: u64 = 0;
    for i in 0..move_count {
        let mut updated_pos = *pos;
        updated_pos.make(&move_list[i]);

        let nodes: u64 = perft(&updated_pos, depth-1);

        total_count += nodes;
    }

    return total_count;
}

pub fn check_movegen_correctness() {
    for p_res in PERFT_RESULTS {
        let mut pos = Position::new();
        pos.parse_fen(p_res.fen);

        assert_eq!(p_res.move_count, perft(&pos, p_res.depth));
        println!("Test '{}' passed", p_res.fen);
    }
    println!("Movegen passed");
}