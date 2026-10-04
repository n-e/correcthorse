use crate::chess::{Move, Position};

pub fn engine(pos: &Position) -> Move {
    let moves = pos.pseudo_legal_moves();

    let legal_moves: Vec<_> = moves
        .iter()
        .filter(|m| {
            let mut p2 = pos.clone();
            p2.play(m);
            p2.is_fully_legal()
        })
        .collect();

    legal_moves[0].clone()
}
