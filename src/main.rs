mod common;
mod day_1;
mod day_2;
mod day_3;
mod day_4;

use std::env::args;
use std::fs::read_to_string;

static PUZZLES: &[[fn(&str) -> usize; 2]] = &[
    [day_1::puzzle_1, day_1::puzzle_2],
    [day_2::puzzle_1, day_2::puzzle_2],
    [day_3::puzzle_1, day_3::puzzle_2],
    [day_4::puzzle_1, day_4::puzzle_2],
];

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();

    let day = args[0].parse::<usize>().unwrap();

    let part = args[1].parse::<usize>().unwrap();

    let input = read_to_string(format!("input/{day}-{part}.txt")).unwrap();

    let puzzle = PUZZLES[day - 1][part - 1];

    let result = puzzle(&input);
    println!("{result}");
}
