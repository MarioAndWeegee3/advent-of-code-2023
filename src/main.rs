mod common;
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

use std::env::args;
use std::fs::read_to_string;

static PUZZLES: &[[fn(&str) -> usize; 2]] = &[
    [day_1::puzzle_1, day_1::puzzle_2],
    [day_2::puzzle_1, day_2::puzzle_2],
    [day_3::puzzle_1, day_3::puzzle_2],
    [day_4::puzzle_1, day_4::puzzle_2],
    [day_5::puzzle_1, day_5::puzzle_2],
    [day_6::puzzle_1, day_6::puzzle_2],
    [day_7::puzzle_1, day_7::puzzle_2],
    [day_8::puzzle_1, day_8::puzzle_2],
    [day_9::puzzle_1, day_9::puzzle_2],
];

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();

    let day = args[0].parse::<usize>().unwrap();

    let part = args[1].parse::<usize>().unwrap();

    let input = read_to_string(format!("input/{day}.txt")).unwrap();

    let puzzle = PUZZLES[day - 1][part - 1];

    let result = puzzle(&input);
    println!("{result}");
}
