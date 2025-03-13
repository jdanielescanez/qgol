mod board;
mod utils;

mod parser;
use parser::read_input;

use std::fs;
use std::fs::File;
use std::io::Write;

fn main() {
    let input_filename = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please, provide the input file as first argument");
        std::process::exit(1)
    });

    let output_filename = std::env::args().nth(2).unwrap_or_else(|| {
        eprintln!("Please, provide the output file as second argument");
        std::process::exit(2)
    });

    let turns = std::env::args()
        .nth(3)
        .unwrap_or_else(|| {
            eprintln!("Please, provide the number of turns as third argument");
            std::process::exit(3)
        })
        .parse::<usize>()
        .unwrap_or_else(|_| {
            eprintln!("The number of turns must be a positive integer");
            std::process::exit(4)
        });

    let board_str =
        fs::read_to_string(input_filename).expect("Should have been able to read the file");
    let (_, mut board) = read_input(&board_str).unwrap();

    for _ in 0..turns {
        board.next();
    }

    let mut output_file = File::create(output_filename).unwrap();

    let output = board
        .get_memory()
        .iter()
        .map(|turn| {
            turn.iter()
                .map(|row| row.iter().map(|cell| cell.prob_alive.to_string()).collect())
                .collect()
        })
        .collect::<Vec<Vec<Vec<String>>>>();

    output_file
        .write_all(ron::to_string(&output).unwrap().as_bytes())
        .unwrap();
}
