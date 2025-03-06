use super::board::Board;

use nom::combinator::map;
use nom::Parser;
use nom::{
    IResult,
    character::complete::{space1, line_ending},
    multi::separated_list1,
    number::complete::double,
};

fn read_row(input: &str) -> IResult<&str, Vec<f64>> {
    separated_list1(space1, double).parse(input)
}

pub fn read_input(input: &str) -> IResult<&str, Board> {
    map(
        separated_list1(line_ending, read_row),
        Board::new,
    ).parse(input)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        assert_eq!(
            read_input(&format!(
                "{}\n{}\n{}\n{}",
                "0.0 0.1 0.2 0.3 0.4",
                "0.5 0.6 0.7 0.8 0.9",
                "1.0 0.1 0.2 0.3 0.4",
                "0.5 0.6 0.7 0.8 0.9"
            )),
            Ok((
                "",
                Board::new(
                    vec![
                        vec![0.0, 0.1, 0.2, 0.3, 0.4],
                        vec![0.5, 0.6, 0.7, 0.8, 0.9],
                        vec![1.0, 0.1, 0.2, 0.3, 0.4],
                        vec![0.5, 0.6, 0.7, 0.8, 0.9],
                    ],
                )
            ))
        );
    }
}