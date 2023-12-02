use crate::common::Lexer;

pub fn puzzle_1(input: &str) -> usize {
    const MAX_RED: usize = 12;
    const MAX_GREEN: usize = 13;
    const MAX_BLUE: usize = 14;

    let mut result = 0;

    for line in input.lines() {
        let game = parse_game(line);
        
        let mut valid = true;

        for set in game.sets.iter() {
            if set.red > MAX_RED || set.blue > MAX_BLUE || set.green > MAX_GREEN {
                valid = false;
                break;
            }
        }

        if valid {
            result += game.id;
        }
    }

    result
}

pub fn puzzle_2(input: &str) -> usize {
    let mut result = 0;
    
    for line in input.lines() {
        let game = parse_game(line);
        
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        
        for set in game.sets.iter() {
            if set.red > min_red {
                min_red = set.red;
            }
            
            if set.green > min_green {
                min_green = set.green;
            }
            
            if set.blue > min_blue {
                min_blue = set.blue;
            }
        }
        
        let power = min_red * min_blue * min_green;
        
        result += power;
    }
    
    result
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Cubes {
    red: usize,
    blue: usize,
    green: usize,
}

#[derive(Default)]
struct Game {
    id: usize,
    sets: Vec<Cubes>,
}

fn parse_game(line: &str) -> Game {
    let mut lexer = Lexer::new(line);

    let mut game = Game::default();

    lexer.advance_matches("Game").unwrap();
    lexer.skip_whitespace();

    game.id = lexer.parse_int().unwrap();

    lexer.skip_whitespace();

    lexer.advance_matches(":").unwrap();

    lexer.skip_whitespace();

    // parse sets
    loop {
        // parse cube numbers
        let mut cubes = Cubes::default();

        while let Some(num) = lexer.parse_int() {
            lexer.skip_whitespace();
            if lexer.advance_matches("red").is_some() {
                cubes.red = num;
            } else if lexer.advance_matches("blue").is_some() {
                cubes.blue = num;
            } else if lexer.advance_matches("green").is_some() {
                cubes.green = num;
            } else {
                panic!("No color found after number")
            }

            lexer.skip_whitespace();

            if lexer.advance_matches(",").is_some() {
                lexer.skip_whitespace();
                continue;
            }

            break;
        }

        game.sets.push(cubes);

        if lexer.advance_matches(";").is_some() {
            lexer.skip_whitespace();
            continue;
        }

        break;
    }
    
    game
}
