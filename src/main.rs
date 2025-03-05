
mod board;
mod utils;
mod parser;
use parser::read_input;
use std::fs;

fn main() {
    let input_filename = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please, provide the input file as argument");
        std::process::exit(1)
    });

    let board_str = fs::read_to_string(input_filename).expect("Should have been able to read the file");
    let (_, mut board) = read_input(&board_str).unwrap();
    dbg!(&board);

    board.next();
    dbg!(&board);
}
