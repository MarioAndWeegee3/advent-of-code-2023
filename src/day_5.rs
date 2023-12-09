use std::ops::Range;

use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::common::Lexer;

pub fn puzzle_1(input: &str) -> usize {
    let mut lexer = Lexer::new(input);

    lexer.advance_matches("seeds:").unwrap();
    lexer.skip_whitespace();

    let mut seeds = Vec::new();

    while let Some(num) = lexer.parse_int() {
        seeds.push(num);
        lexer.skip_whitespace();
    }

    let mut maps = Vec::new();

    while lexer.peek().is_some() {
        maps.push(Map::parse(&mut lexer).unwrap());
    }

    seeds
        .into_iter()
        .map(|seed| maps.iter().fold(seed, |value, map| map.convert(value)))
        .min()
        .unwrap()
}

pub fn puzzle_2(input: &str) -> usize {
    let mut lexer = Lexer::new(input);

    lexer.advance_matches("seeds:").unwrap();
    lexer.skip_whitespace();

    let mut seed_ranges = Vec::new();

    while let Some(start) = lexer.parse_int() {
        lexer.skip_whitespace();
        let len = lexer.parse_int().unwrap();
        lexer.skip_whitespace();
        let range = start..start + len;
        seed_ranges.push(range);
    }

    let seeds = seed_ranges.into_par_iter().flatten();

    let mut maps = Vec::new();

    while lexer.peek().is_some() {
        maps.push(Map::parse(&mut lexer).unwrap());
    }

    seeds
        .map(|seed| maps.iter().fold(seed, |value, map| map.convert(value)))
        .min()
        .unwrap()
}

struct Map {
    mappings: Vec<Mapping>,
}

impl Map {
    fn parse(lexer: &mut Lexer) -> Option<Self> {
        let _source_category = match lexer.parse_word()? {
            "seed" => Category::Seed,
            "soil" => Category::Soil,
            "fertilizer" => Category::Fertilizer,
            "water" => Category::Water,
            "light" => Category::Light,
            "temperature" => Category::Temperature,
            "humidity" => Category::Humidity,
            "location" => Category::Location,
            _ => return None,
        };

        lexer.advance_matches("-to-")?;
        lexer.parse_word()?;
        lexer.skip_whitespace();

        lexer.advance_matches("map:")?;
        lexer.skip_whitespace();

        let mut mappings = Vec::new();

        loop {
            let destination_start = match lexer.parse_int() {
                Some(v) => v,
                None => break,
            };
            lexer.skip_whitespace();

            let source_start = lexer.parse_int()?;
            lexer.skip_whitespace();

            let len = lexer.parse_int()?;
            lexer.skip_whitespace();

            let mapping = Mapping {
                destination_start,
                source_start,
                len,
            };

            mappings.push(mapping);
        }

        Some(Map { mappings })
    }

    fn convert(&self, value: usize) -> usize {
        for mapping in self.mappings.iter() {
            if mapping.source_range().contains(&value) {
                let index = value - mapping.source_start;
                return index + mapping.destination_start;
            }
        }

        value
    }
}

#[derive(Clone, Copy)]
struct Mapping {
    source_start: usize,
    destination_start: usize,
    len: usize,
}

impl Mapping {
    fn source_range(&self) -> Range<usize> {
        self.source_start..self.source_start + self.len
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}
