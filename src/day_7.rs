use std::{
    cmp::{Ordering, Reverse},
    collections::HashMap,
    fmt::Display,
};

use crate::common::Lexer;

pub fn puzzle_1(source: &str) -> usize {
    puzzle_common(source, false)
}

pub fn puzzle_2(source: &str) -> usize {
    puzzle_common(source, true)
}

fn puzzle_common(source: &str, include_jokers: bool) -> usize {
    let mut hands = source
        .lines()
        .map(|line| parse_hand(&mut Lexer::new(line), include_jokers))
        .collect::<Option<Vec<_>>>()
        .unwrap();

    hands.sort_by_key(|(h, _)| *h);

    hands
        .into_iter()
        .enumerate()
        .map(|(index, (hand, bid))| {
            let rank = index + 1;
            let win = rank * bid;
            let _type = hand.get_type();
            println!("{rank:>4} * {bid:>3} {win:>6} {hand} {_type:?}");
            win
        })
        .sum::<usize>()
}

fn parse_hand(lexer: &mut Lexer, include_jokers: bool) -> Option<(Hand, usize)> {
    let mut cards = [Card::Two; 5];

    for card in cards.iter_mut() {
        *card = match lexer.advance()? {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => {
                if include_jokers {
                    Card::Joker
                } else {
                    Card::Jack
                }
            }
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => return None,
        }
    }

    let hand = Hand { cards };
    lexer.skip_whitespace();

    let bid = lexer.parse_int()?;

    Some((hand, bid))
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for card in self.cards {
            let str = match card {
                Card::Joker => "J",
                Card::Two => "2",
                Card::Three => "3",
                Card::Four => "4",
                Card::Five => "5",
                Card::Six => "6",
                Card::Seven => "7",
                Card::Eight => "8",
                Card::Nine => "9",
                Card::Ten => "T",
                Card::Jack => "J",
                Card::Queen => "Q",
                Card::King => "K",
                Card::Ace => "A",
            };
            f.write_str(str)?;
        }

        Ok(())
    }
}

impl Hand {
    fn get_type(&self) -> HandType {
        let mut card_counts: HashMap<Card, u8> = HashMap::new();

        for card in self.cards {
            let count = card_counts.entry(card).or_default();
            *count += 1;
        }

        if let Some(&value) = card_counts.get(&Card::Joker) {
            if value == 5 {
                return HandType::FiveOfAKind;
            }

            let highest_value = card_counts
                .iter_mut()
                .filter_map(|(k, v)| (*k != Card::Joker).then_some(v))
                .max()
                .unwrap();

            *highest_value += value;

            card_counts.remove(&Card::Joker);
        }

        let mut card_counts = card_counts.into_values().collect::<Vec<_>>();

        card_counts.sort_by_key(|c| Reverse(*c));

        match card_counts.as_slice() {
            [5] => HandType::FiveOfAKind,
            [4, 1] => HandType::FourOfAKind,
            [3, 2] => HandType::FullHouse,
            [3, 1, 1] => HandType::ThreeOfAKind,
            [2, 2, 1] => HandType::TwoPair,
            [2, 1, 1, 1] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let (l_type, r_type) = (self.get_type(), other.get_type());

        (l_type, &self.cards).cmp(&(r_type, &other.cards))
    }
}
