use crate::chess::{Move, Position};

pub fn perft(p: &mut Position, depth: i64) -> i64 {
    if depth == 0 {
        return 1;
    }

    let legal_moves = p.pseudo_legal_moves();

    legal_moves
        .iter()
        .map(|mov| {
            let mut p2 = p.clone();
            // println!("{} {:?}", 2 - depth, mov);

            p2.play(mov);

            if !p2.is_fully_legal() {
                return 0;
            }

            let n = perft(&mut p2, depth - 1);
            // p.undo();
            n
        })
        .sum()
}

pub fn debug_perft(p: &mut Position, depth: i64, prefix: Vec<Move>) -> i64 {
    if depth == 0 {
        println!(
            "{}",
            prefix
                .iter()
                .map(|m| m.to_lan())
                .collect::<Vec<_>>()
                .join(" ")
        );
        return 1;
    }

    let legal_moves = p.pseudo_legal_moves();

    legal_moves
        .iter()
        .map(|mov| {
            let mut p2 = p.clone();

            p2.play(mov);

            // println!("{:?} {}", mov, p2.is_fully_legal());

            if !p2.is_fully_legal() {
                return 0;
            }

            let mut pref = prefix.clone();
            pref.push(mov.clone());
            let n = debug_perft(&mut p2, depth - 1, pref);
            // p.undo();
            n
        })
        .sum()
}
