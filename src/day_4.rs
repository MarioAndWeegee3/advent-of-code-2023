use std::rc::Rc;

use crate::common::Lexer;

pub fn puzzle_1(source: &str) -> usize {
    let mut result = 0;

    for line in source.lines() {
        let card = ScratchCard::parse(line).unwrap();

        let mut points = 0;

        for number in card.have.iter() {
            if card.winning.contains(number) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }

        result += points;
    }

    result
}

pub fn puzzle_2(source: &str) -> usize {
    let mut cards = source
        .lines()
        .map(|line| ScratchCard::parse(line).unwrap())
        .collect::<Vec<_>>();

    let mut index = 0;

    while let Some(card) = cards.get(index).cloned() {
        let matches = card
            .have
            .iter()
            .filter(|n| card.winning.contains(n))
            .count();

        for i in card.id + 1..=card.id + matches {
            if let Some(c) = cards.iter().find(|c| c.id == i).cloned() {
                cards.push(c);
            }
        }

        index += 1;
    }

    cards.len()
}

#[derive(Clone)]
struct ScratchCard {
    id: usize,
    winning: Rc<[u8]>,
    have: Rc<[u8]>,
}

impl ScratchCard {
    pub fn parse(line: &str) -> Option<Self> {
        let mut lexer = Lexer::new(line);
        lexer.advance_matches("Card")?;
        lexer.skip_whitespace();

        let id = lexer.parse_int()?;
        lexer.skip_whitespace();

        lexer.advance_matches(":")?;
        lexer.skip_whitespace();

        let mut winning = Vec::new();
        let mut have = Vec::new();

        let mut target = &mut winning;

        while let Some(c) = lexer.peek() {
            if c == '|' {
                lexer.advance();
                target = &mut have;
                lexer.skip_whitespace();
            } else {
                let value = lexer.parse_int()?;
                lexer.skip_whitespace();

                target.push(value as u8);
            }
        }

        let winning = Rc::from(winning);
        let have = Rc::from(have);

        Some(Self { id, winning, have })
    }
}
