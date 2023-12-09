use rayon::prelude::*;

use crate::common::Lexer;

pub fn puzzle_1(source: &str) -> usize {
    puzzle_common(source, false) as usize
}

pub fn puzzle_2(source: &str) -> usize {
    puzzle_common(source, true) as usize
}

fn puzzle_common(source: &str, beginning: bool) -> isize {
    let sequences = parse_sequences(source);

    sequences
        .into_par_iter()
        .map(|mut initial_sequence| {
            if beginning {
                initial_sequence.reverse();
            }
            
            let mut differences = vec![initial_sequence];

            let mut diff_index = 0;

            loop {
                let last_diff = differences[diff_index].as_slice();
                diff_index += 1;
                let mut diff = Vec::new();

                let mut last = last_diff[0];
                for i in 1..last_diff.len() {
                    let current = last_diff[i];
                    diff.push(current - last);
                    last = current;
                }

                let zeroes = diff.iter().all(|v| *v == 0);

                differences.push(diff);

                if zeroes {
                    break;
                }
            }

            for i in (0..differences.len()).rev() {
                if i == differences.len() - 1 {
                    differences[i].push(0);
                } else {
                    let &last_v = differences[i].last().unwrap();
                    let &last_diff = differences[i + 1].last().unwrap();
                    let v = last_v + last_diff;
                    differences[i].push(v);
                }
            }

            differences[0].last().copied().unwrap()
        })
        .sum::<isize>()
}

fn parse_sequences(source: &str) -> Vec<Vec<isize>> {
    source
        .lines()
        .map(|source| {
            let mut lexer = Lexer::new(source);
            parse_sequence(&mut lexer)
        })
        .collect()
}

fn parse_sequence(lexer: &mut Lexer) -> Vec<isize> {
    let mut result = Vec::new();
    
    while let Some(value) = lexer.parse_signed_int() {
        result.push(value);
        lexer.skip_whitespace();
    }
    
    result
}