mod chess;
mod engine;
mod perft;
mod util;
use std::io::{self, BufRead};

use clap::Parser;

use perft::perft;

use crate::{
    chess::{Move, Position},
    engine::engine,
    perft::debug_perft,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "depth")]
    perft: Option<u8>,

    #[arg(short, long, value_name = "depth")]
    search: Option<u8>,

    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    #[arg(short, long)]
    epd: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let mut pos = cli
        .epd
        .map_or(Position::start_pos(), |str| Position::from_epd(str));

    if let Some(depth) = cli.perft {
        if cli.verbose {
            debug_perft(&mut pos, depth.into(), vec![]);
        } else {
            let nodes = perft(&mut pos, depth.into());
            println!("depth {} nodes {}", depth, nodes)
        }
    } else if let Some(depth) = cli.search {
        let res = engine(&pos, depth as i16);
        println!(
            "bestmove {} eval {}",
            res.0.map_or("-".to_string(), |m| m.to_lan()),
            res.1 / 100
        );
    } else {
        uci();
    }
}

fn uci() {
    let stdin = io::stdin();

    let mut position = Position::start_pos();

    for _line in stdin.lock().lines() {
        let line = _line.unwrap();

        if line == "uci" {
            println!("uciok")
        } else if line == "isready" {
            println!("readyok")
        } else if line == "quit" {
            break;
        } else if line.starts_with("position") {
            // position startpos moves b1a3

            let spl: Vec<_> = line.split(" ").collect();
            let moves_idx = spl.iter().position(|p| *p == "moves");
            let moves = moves_idx.map_or(vec![], |idx| spl[idx + 1..].to_vec());

            position = Position::start_pos();
            for m in moves {
                position.play(&Move::from_lan(m.to_string()));
            }
        } else if line.starts_with("go") {
            // go wtime 9999 btime 10000 movestogo 10 depth 1
            let spl: Vec<_> = line.split(" ").collect();
            let depth_idx = spl.iter().position(|p| *p == "depth").unwrap();
            let depth: i16 = spl[depth_idx + 1].parse().unwrap();

            let mov = engine(&position, depth);

            println!("bestmove {}", mov.0.unwrap().to_lan());
        }
    }
}
