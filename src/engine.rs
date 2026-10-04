use crate::chess::{Move, Piece, Position};

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

pub fn engine(pos: &Position, depth: i16) -> (Vec<Move>, i64) {
    if depth == 0 {
        return (vec![], sign(pos.side) * eval(pos));
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
            return (vec![], -100 * 1000);
        } else {
            return (vec![], 0);
        }
    }

    let mut eval = i64::MIN;
    let mut selected_pv: Vec<Move> = vec![];
    for mov in legal_moves {
        let mut p2 = pos.clone();
        p2.play(mov);

        let mut res = engine(&p2, depth - 1);
        res.1 = -res.1;

        if res.1 > eval {
            eval = res.1;
            selected_pv = [vec![mov.clone()], res.0].concat();
        }
    }

    (selected_pv, eval)
}
