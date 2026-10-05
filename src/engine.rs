use std::{
    cmp,
    time::{Duration, Instant},
};

use crate::chess::{Move, Piece, Position};

const MIN_EVAL: i64 = -424242 * 100;
const MATE_0_EVAL: i64 = 1000 * 100;
const MAX_EVAL: i64 = 424242 * 100;

fn piece_score(piece: Piece) -> i64 {
    match piece {
        Piece::King => 0,
        Piece::Queen => 900,
        Piece::Rook => 500,
        Piece::Bishop => 300,
        Piece::Knight => 300,
        Piece::Pawn => 100,
    }
}

pub fn eval(pos: &Position) -> i64 {
    let rnd: i64 = rand::random_range(-5..5);

    let piece_eval: i64 = pos
        .board
        .iterate_squares()
        .map(|x| {
            if let Some((s, p, c)) = x {
                sign(c) * piece_score(p)
            } else {
                0
            }
        })
        .sum();

    piece_eval + rnd
}

// function negamax(node, depth, color) is
//     if depth = 0 or node is a terminal node then
//         return color × the heuristic value of node
//     value := −∞
//     for each child of node do
//         value := max(value, −negamax(child, depth − 1, −color))
//     return value

fn sign(color: crate::chess::Color) -> i64 {
    match color {
        crate::chess::Color::White => 1,
        crate::chess::Color::Black => -1,
    }
}

// returns None if interrupted
pub fn negamax(
    pos: &Position,
    depth: i16,
    alpha: i64,
    beta: i64,
    globals: &mut EngineGlobals,
) -> Option<(Vec<Move>, i64)> {
    if depth == 0 {
        globals.nodes += 1;

        if globals.nodes % 10_000 == 0
            && globals
                .allowed_time
                .is_some_and(|at| globals.start_time.elapsed() > at)
        {
            return None;
        }

        return Some((vec![], sign(pos.side) * eval(pos)));
    }

    let moves = pos.pseudo_legal_moves();
    let legal_moves: Vec<_> = moves
        .iter()
        .filter(|m| {
            let mut p2 = pos.clone();
            p2.play(m);
            p2.is_fully_legal()
        })
        .collect();

    if legal_moves.len() == 0 {
        if pos.board.is_in_check(pos.side) {
            return Some((vec![], -MATE_0_EVAL));
        } else {
            return Some((vec![], 0));
        }
    }

    let mut eval = MIN_EVAL;
    let mut selected_pv: Vec<Move> = vec![];
    let mut updated_alpha = alpha;
    for mov in legal_moves {
        let mut p2 = pos.clone();
        p2.play(mov);

        let res = negamax(&p2, depth - 1, -beta, -updated_alpha, globals);

        if let Some(mut res) = res {
            res.1 = -res.1;

            // eprintln!("{}",)

            if res.1 > eval {
                eval = res.1;
                selected_pv = [vec![mov.clone()], res.0].concat();
            }

            updated_alpha = cmp::max(eval, updated_alpha);
            if updated_alpha >= beta {
                break;
            }
        } else {
            return None;
        }
    }

    if eval > MATE_0_EVAL - 100 {
        eval = eval - 5
    }
    if eval < -MATE_0_EVAL + 100 {
        eval = eval + 5
    }

    Some((selected_pv, eval))
}

pub struct EngineOpts {
    pub wtime: u64,
    pub btime: u64,
    pub winc: u64,
    pub binc: u64,
    pub depth: i16,
}

pub struct EngineGlobals {
    nodes: i64,
    start_time: Instant,
    allowed_time: Option<Duration>,
}

pub fn engine(pos: &Position, opts: &EngineOpts) -> (Vec<Move>, i64) {
    let mut globals = EngineGlobals {
        nodes: 0,
        start_time: Instant::now(),
        allowed_time: None,
    };

    match pos.side {
        crate::chess::Color::White => {
            if opts.wtime > 0 {
                globals.allowed_time = Some(Duration::from_millis(opts.wtime / 5))
            }
        }
        crate::chess::Color::Black => {
            if opts.btime > 0 {
                globals.allowed_time = Some(Duration::from_millis(opts.btime / 5))
            }
        }
    }
    // println!("{:?}", globals.allowed_time);

    let mut depth: i16 = 1;
    let mut best = None;
    loop {
        let ret = negamax(pos, depth, MIN_EVAL, MAX_EVAL, &mut globals);
        // println!("{:?} {}", ret, depth);

        if let Some(ret) = ret {
            println!(
                "info depth {} score cp {} time {} nodes {} nps {} pv {}",
                depth,
                ret.1,
                globals.start_time.elapsed().as_millis(),
                globals.nodes,
                globals.nodes as f64 / globals.start_time.elapsed().as_secs_f64(),
                ret.0
                    .iter()
                    .map(|x| x.to_lan())
                    .collect::<Vec<_>>()
                    .join(" "),
            );

            best = Some(ret);
        } else {
            return best.unwrap();
        }

        if opts.depth > 0 && depth >= opts.depth {
            return best.unwrap();
        }

        depth += 1;
    }
}
