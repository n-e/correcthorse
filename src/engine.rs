use crate::chess::{Move, Position};

pub fn eval(pos: &Position) -> i64 {
    return rand::random_range(-5..5);
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

pub fn engine(pos: &Position, depth: i16) -> (Option<Move>, i64) {
    if depth == 0 {
        return (None, sign(pos.side) * eval(pos));
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
            return (None, -100 * 1000);
        } else {
            return (None, 0);
        }
    }

    let mut eval = i64::MIN;
    let mut selected_move: Option<Move> = None;
    for mov in legal_moves {
        let mut p2 = pos.clone();
        p2.play(mov);

        let mut res = engine(&p2, depth - 1);
        res.1 = -res.1;

        if res.1 > eval {
            eval = res.1;
            selected_move = Some(mov.clone());
        }
    }

    (selected_move, eval)
}
