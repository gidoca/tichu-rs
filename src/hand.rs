use crate::card::*;
use crate::util::iter_all_equal;

use itertools::Itertools;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum HandType {
    SingleCard,
    Pair,
    Triple,
    Straight,
    StraightOfPairs,
    FullHouse,
    QuadrupleBomb,
    StraightBomb,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Hand(Vec<Card>);

impl Hand {
    pub fn new(mut cards: Vec<Card>) -> Hand {
        cards.sort_unstable();
        Hand(cards)
    }

    pub fn single_card(card: Card) -> Hand {
        Hand(vec![card])
    }

    pub fn hand_type(&self) -> Option<HandType> {
        if self.0.len() > 1 && !self.0.iter().all(|card| card.is_valid_in_multi_card_hand()) {
            return None;
        }

        match self {
            hand if hand.is_valid_quadruple_bomb() => Some(HandType::QuadrupleBomb),
            hand if hand.is_valid_straight_bomb() => Some(HandType::StraightBomb),
            hand if hand.is_valid_single_card() => Some(HandType::SingleCard),
            hand if hand.is_valid_pair() => Some(HandType::Pair),
            hand if hand.is_valid_triple() => Some(HandType::Triple),
            hand if hand.is_valid_straight() => Some(HandType::Straight),
            hand if hand.is_valid_straight_of_pairs() => Some(HandType::StraightOfPairs),
            hand if hand.is_valid_full_house() => Some(HandType::FullHouse),
            _ => None,
        }
    }

    pub fn is_valid_quadruple_bomb(&self) -> bool {
        match self.0.as_slice() {
            [Card::RegularCard(_, _), Card::RegularCard(_, _), Card::RegularCard(_, _), Card::RegularCard(_, _)] => {
                iter_all_equal(self.0.iter().map(|card| card.value())).is_some()
            }
            _ => false,
        }
    }

    pub fn is_valid_straight_bomb(&self) -> bool {
        self.is_valid_straight()
            && self.num_phoenices() == 0
            && iter_all_equal(self.0.iter().map(|card| card.suite())).is_some()
    }

    pub fn is_valid_single_card(&self) -> bool {
        self.0.len() == 1
    }

    pub fn is_valid_pair(&self) -> bool {
        match self.0.as_slice() {
            [Card::RegularCard(_, _), Card::RegularCard(_, _)] => {
                iter_all_equal(self.0.iter().map(|card| card.value())).is_some()
            }
            [Card::SpecialCard(SpecialCardType::Phoenix), Card::RegularCard(_, _)] => true,
            _ => false,
        }
    }

    pub fn is_valid_triple(&self) -> bool {
        match self.0.as_slice() {
            [Card::RegularCard(value1, _), Card::RegularCard(value2, _), Card::RegularCard(value3, _)]
                if value1 == value2 && value2 == value3 =>
            {
                true
            }
            [Card::SpecialCard(SpecialCardType::Phoenix), Card::RegularCard(value1, _), Card::RegularCard(value2, _)]
                if value1 == value2 =>
            {
                true
            }
            _ => false,
        }
    }

    pub fn is_valid_straight(&self) -> bool {
        if self.0.as_slice().windows(2).any(|pair| {
            pair[0]
                .numeric_value()
                .zip(pair[1].numeric_value())
                .map_or(false, |(left, right)| left == right)
        }) {
            return false;
        }
        let num_phoenices = self.num_phoenices();
        let num_phoenices_needed = self
            .0
            .as_slice()
            .windows(2)
            .filter_map(|cards| match cards {
                [card1, card2] => match (card1.numeric_value(), card2.numeric_value()) {
                    (Some(value1), Some(value2)) => Some(value2 - value1 - 1),
                    _ => None,
                },
                _ => panic!(),
            })
            .sum();
        self.0.len() >= 5 && num_phoenices >= num_phoenices_needed
    }

    pub fn is_valid_straight_of_pairs(&self) -> bool {
        let num_phoenices = self.num_phoenices();
        let mut num_phoenices_needed = 0;
        let first_value = self.0.iter().filter_map(|card| card.numeric_value()).next();
        let last_value = self
            .0
            .iter()
            .map(|card| card.numeric_value().unwrap())
            .next_back();

        let Some(first_value) = first_value else {
            return false;
        };
        let Some(last_value) = last_value else {
            return false;
        };

        const NUM_CARDS: usize = 2;

        let num_cards_by_value = self
            .0
            .iter()
            .chunk_by(|card| card.numeric_value())
            .into_iter()
            .filter(|(value, _)| value.is_some())
            .filter_map(|(value, cards)| match cards.count() {
                length if length > NUM_CARDS => None,
                length => Some((value, length)),
            })
            .collect::<HashMap<_, _>>();

        for value in first_value..=last_value {
            let num_cards_for_current_value = num_cards_by_value
                .get(&Some(value))
                .unwrap_or(&(0 as usize));
            if *num_cards_for_current_value > NUM_CARDS {
                return false;
            }
            num_phoenices_needed += NUM_CARDS - num_cards_for_current_value;
        }

        num_phoenices >= num_phoenices_needed
    }

    pub fn is_valid_full_house(&self) -> bool {
        match self.0.as_slice() {
            [Card::RegularCard(value1, _), Card::RegularCard(value2, _), Card::RegularCard(value3, _), Card::RegularCard(value4, _), Card::RegularCard(value5, _)] => {
                value1 == value2 && value4 == value5 && (value3 == value2 || value3 == value4)
            }
            [Card::SpecialCard(SpecialCardType::Phoenix), Card::RegularCard(value1, _), Card::RegularCard(value2, _), Card::RegularCard(value3, _), Card::RegularCard(value4, _)] => {
                ((value1 == value2 && value2 == value3)
                    || (value1 == value2 && value3 == value4)
                    || (value2 == value3 && value3 == value4))
                    && value1 != value4
            }
            _ => false,
        }
    }

    pub fn num_phoenices(&self) -> usize {
        self.0
            .iter()
            .filter(|card| match card {
                Card::SpecialCard(SpecialCardType::Phoenix) => true,
                _ => false,
            })
            .count()
    }

    pub fn relevant_card_value(&self) -> Option<RegularCardValue> {
        self.hand_type()
            .map(|hand_type| match hand_type {
                HandType::SingleCard
                | HandType::Pair
                | HandType::Triple
                | HandType::Straight
                | HandType::StraightOfPairs
                | HandType::QuadrupleBomb
                | HandType::StraightBomb => self.0.iter().next_back().unwrap(),
                HandType::FullHouse => match self.0[0] {
                    Card::RegularCard(_, _) => &self.0[2],
                    Card::SpecialCard(SpecialCardType::Phoenix) => &self.0[3],
                    _ => panic!(),
                },
            })
            .map(|card| match card {
                Card::RegularCard(value, _) => *value,
                _ => panic!(),
            })
    }

    pub fn is_bomb(&self) -> bool {
        match self.hand_type() {
            Some(HandType::StraightBomb) | Some(HandType::QuadrupleBomb) => true,
            _ => false,
        }
    }

    pub fn higher_value_than(&self, other: &Hand) -> bool {
        self.relevant_card_value()
            .zip(other.relevant_card_value())
            .map(|(self_value, other_value)| self_value > other_value)
            .unwrap_or(false)
    }

    pub fn can_be_played_on(&self, other: &Hand) -> bool {
        if self.is_bomb() {
            if other.is_bomb() {
                self.0.len() > other.0.len()
                    || (self.0.len() == other.0.len() && self.higher_value_than(other))
            } else {
                true
            }
        } else {
            self.hand_type() == other.hand_type()
                && self.0.len() == other.0.len()
                && self.higher_value_than(other)
        }
    }
}
