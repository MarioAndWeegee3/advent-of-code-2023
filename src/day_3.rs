use crate::common::{Grid, Lexer};

pub fn puzzle_1(source: &str) -> usize {
    let mut result = 0;

    let width = source.lines().next().unwrap().len();

    let bytes = source
        .lines()
        .flat_map(|l| l.trim().bytes())
        .collect::<Vec<_>>();

    let grid = Grid::new(&bytes, width);

    let numbers = get_numbers(source);

    for Number { x, y, len, value } in numbers {
        let mut is_part_num = false;

        let min_x = x.saturating_sub(1);
        let min_y = y.saturating_sub(1);

        let max_x = (x + len + 1).min(grid.width());
        let max_y = (y + 2).min(grid.height());

        for y in min_y..max_y {
            for x in min_x..max_x {
                let c = grid.get(x, y);
                if is_symbol(c) {
                    is_part_num = true;
                }
            }
        }

        if is_part_num {
            println!("{x}, {y}: {value}");
            result += value;
        }
    }

    result
}

pub fn puzzle_2(source: &str) -> usize {
    let mut result = 0;

    let width = source.lines().next().unwrap().len();

    let bytes = source
        .lines()
        .flat_map(|l| l.trim().bytes())
        .collect::<Vec<_>>();

    let grid = Grid::new(&bytes, width);

    let numbers = get_numbers(source);

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let c = grid.get(x, y);
            if c == b'*' {
                let numbers = numbers
                    .iter()
                    .filter(|num| {
                        let min_x = x.saturating_sub(1);
                        let min_y = y.saturating_sub(1);
                        let max_x = (x + 2).min(grid.width());
                        let max_y = (y + 2).min(grid.height());

                        let mut result = false;

                        for y in min_y..max_y {
                            for x in min_x..max_x {
                                if num.is_in_bounds(x, y) {
                                    result = true;
                                }
                            }
                        }

                        result
                    })
                    .collect::<Vec<_>>();

                if numbers.len() == 2 {
                    result += numbers[0].value * numbers[1].value;
                }
            }
        }
    }

    result
}

fn get_numbers(source: &str) -> Vec<Number> {
    let mut numbers = Vec::new();

    for (y, line) in source.lines().enumerate() {
        let mut lexer = Lexer::new(line);

        let mut index = lexer.index();

        while lexer.peek().is_some() {
            if let Some(value) = lexer.parse_int() {
                let x = index;
                index = lexer.index();
                let len = index - x;
                numbers.push(Number { x, y, len, value });
            } else {
                lexer.advance();
                index = lexer.index();
            }
        }
    }

    numbers
}

struct Number {
    x: usize,
    y: usize,
    len: usize,
    value: usize,
}

impl Number {
    fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        x >= self.x && x < self.x + self.len && y == self.y
    }
}

fn is_symbol(b: u8) -> bool {
    b.is_ascii_punctuation() && b != b'.'
}
