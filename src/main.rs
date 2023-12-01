mod day_1;

use std::env::args;
use std::fs::read_to_string;
 
fn main() {
    let args = args().skip(1).collect::<Vec<_>>();

    let day = args[0].parse::<u8>().unwrap();
    
    let part = args[1].parse::<u8>().unwrap();

    let input = read_to_string(format!("input/{day}-{part}.txt")).unwrap();

    match (day, part) {
        (1, 1) => {
            let result = day_1::puzzle_1(&input);
            println!("{result}");
        }
        (1, 2) => {
            let result = day_1::puzzle_2(&input);
            println!("{result}");
        }
        _ => panic!("Unimplemented")
    }
}
