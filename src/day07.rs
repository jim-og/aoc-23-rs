use std::{cmp::Ordering, collections::HashSet};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Eq, PartialEq)]
struct Hand {
    cards: Vec<u32>,
    hand_type: HandType,
    bid: u32,
}

impl Hand {
    pub fn new(input: &str, jokers: bool) -> Self {
        let split = input.split_whitespace().collect::<Vec<&str>>();
        let cards = split
            .first()
            .expect("Expected cards input")
            .chars()
            .map(|c| match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => {
                    if jokers {
                        1
                    } else {
                        11
                    }
                }
                'T' => 10,
                _ => c.to_digit(10).expect("Unable to parse card number"),
            })
            .collect::<Vec<u32>>();
        let bid = split
            .last()
            .expect("Unable to extract bid")
            .parse::<u32>()
            .expect("Unable to parse bid");

        // Don't add jokers to the set
        let set = cards.iter().filter(|c| **c != 1).collect::<HashSet<&u32>>();

        let hand_type = match set.len() {
            0 => {
                if jokers {
                    HandType::FiveOfAKind
                } else {
                    panic!("Invalid hand")
                }
            }
            1 => HandType::FiveOfAKind,
            2 => {
                // Could either be a FullHouse (AAA KK) or FourOfAKind (AAAA k)
                let mut t = HandType::FullHouse;
                for card in set {
                    if cards.iter().filter(|c| *c == card || **c == 1).count() == 4 {
                        t = HandType::FourOfAKind;
                        break;
                    }
                }
                t
            }
            3 => {
                // Could either be TwoPair (AA KK Q) or ThreeOfAKind (AAA K Q)
                let mut t = HandType::TwoPair;
                for card in set {
                    if cards.iter().filter(|c| *c == card || **c == 1).count() == 3 {
                        t = HandType::ThreeOfAKind;
                        break;
                    }
                }
                t
            }
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => panic!("Invalid hand"),
        };
        Hand {
            cards,
            hand_type,
            bid,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                for i in 0..5 {
                    match self.cards[i].cmp(&other.cards[i]) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Equal => (),
                    }
                }
                Ordering::Equal
            }
            Ordering::Greater => Ordering::Greater,
        }
    }
}

pub fn day07(input: Vec<String>) -> (String, String) {
    let mut part_1_hands = input
        .iter()
        .map(|line| Hand::new(line, false))
        .collect::<Vec<Hand>>();

    part_1_hands.sort();

    let part_1 = part_1_hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + hand.bid * (i as u32 + 1));

    let mut part_2_hands = input
        .iter()
        .map(|line| Hand::new(line, true))
        .collect::<Vec<Hand>>();

    part_2_hands.sort();

    let part_2 = part_2_hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + hand.bid * (i as u32 + 1));

    (format!("{}", part_1), format!("{}", part_2))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    #[test]
    fn example() {
        let result = day07(parser::test_input(
            "32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483",
        ));
        assert_eq!(result.0, "6440");
        assert_eq!(result.1, "5905");
    }

    #[test]
    fn mainline() {
        let input = parser::load_input(7);
        let result = day07(input);
        assert_eq!(result.0, "247815719");
        assert_eq!(result.1, "248747492");
    }
}
