use crate::common::Lexer;

pub fn puzzle_1(input: &str) -> usize {
    let races = parse_races(input, |lexer| lexer.parse_int()).unwrap();
    
    races.into_iter()
        .map(|Race { time, distance_record }| {
            (0..time)
                .into_iter()
                .filter_map(|held| {
                    let speed = held;
                    let remaining_time = time - held;
                    
                    let distance = speed * remaining_time;
                    
                    if distance > distance_record {
                        Some(1)
                    } else {
                        None
                    }
                })
                .sum::<usize>()
        })
        .fold(1, |a, b| a * b)
}

pub fn puzzle_2(input: &str) -> usize {
    let Race { time, distance_record } = parse_races(input, parse_int_with_spaces).unwrap()[0];
    
    (0..time)
        .into_iter()
        .filter_map(|held| {
            let speed = held;
            let remaining_time = time - held;

            let distance = speed * remaining_time;

            if distance > distance_record {
                Some(1)
            } else {
                None
            }
        })
        .sum::<usize>()
}

#[derive(Clone, Copy, Debug)]
struct Race {
    time: usize,
    distance_record: usize,
}

fn parse_races(source: &str, parse_int: fn(&mut Lexer) -> Option<usize>) -> Option<Vec<Race>> {
    let mut result = Vec::new();

    let mut lexer = Lexer::new(source);

    lexer.advance_matches("Time:")?;
    lexer.skip_whitespace();

    while let Some(time) = parse_int(&mut lexer) {
        lexer.skip_whitespace();
        result.push(time)
    }

    lexer.advance_matches("Distance:")?;
    lexer.skip_whitespace();

    result
        .into_iter()
        .map(|time| {
            let distance_record = parse_int(&mut lexer)?;
            lexer.skip_whitespace();

            Some(Race {
                time,
                distance_record,
            })
        })
        .collect()
}

fn parse_int_with_spaces(lexer: &mut Lexer) -> Option<usize> {
    let mut result = 0;
    let mut digits = 0;
    
    while let Some(c) = lexer.peek() {
        if c == ' ' {
            lexer.advance();
        } else if c.is_ascii_digit() {
            lexer.advance();
            result *= 10;
            result += ((c as u8) - b'0') as usize;
            digits += 1;
        } else {
            break;
        }
    }
    
    if digits == 0 {
        return None;
    }
    
    Some(result)
}