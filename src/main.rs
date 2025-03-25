mod board;

mod parser;
use parser::read_input;

use std::fs;
use std::fs::File;
use std::io::Write;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the file with the initial generation
    #[arg(index = 1)]
    input_filename: std::path::PathBuf,
    /// Name of the file to save the memory of the game
    #[arg(index = 2)]
    output_filename: std::path::PathBuf,
    /// Numbers of turns that will be played
    #[arg(index = 3)]
    turns: usize,
    /// Set of alive neighbourhoods sizes to survive in the next turn
    #[arg(short, long, default_value_t = ("[2, 3]").to_string())]
    survivals: String,
    /// Set of alive neighbourhoods sizes to revive in the next turn
    #[arg(short, long, default_value_t = ("[3]").to_string())]
    revivals: String,
}

fn main() {
    let args = Args::parse();

    let board_str =
        fs::read_to_string(args.input_filename).expect("Should have been able to read the file");
    let (_, mut board) = read_input(&board_str).unwrap();

    let survival_rules = serde_json::from_str(&args.survivals).unwrap_or_else(|_| {
        eprintln!("Please, provide the survivals in a serializable Vec<usize> way");
        std::process::exit(1)
    });
    let revival_rules = serde_json::from_str(&args.revivals).unwrap_or_else(|_| {
        eprintln!("Please, provide the revivals in a serializable Vec<usize> way");
        std::process::exit(1)
    });
    board.change_rules(survival_rules, revival_rules);
    let mut memory = vec![board.get_state().clone()];

    for _ in 0..args.turns {
        board.next();
        memory.push(board.get_state().clone());
    }

    let mut output_file = File::create(args.output_filename).unwrap();

    output_file
        .write_all(serde_json::to_string_pretty(&memory).unwrap().as_bytes())
        .unwrap();
}
