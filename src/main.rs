use enum_iterator::{all, Sequence};
use itertools::Itertools;
use rand::{seq::SliceRandom, SeedableRng};
use std::collections::HashMap;

fn iter_all_equal<T: PartialEq>(iter: impl IntoIterator<Item = T>) -> Option<T> {
    let mut iter = iter.into_iter();
    let first = iter.next()?;
    iter.all(|elem| elem == first).then(|| first)
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone, Sequence)]
enum RegularCardValue {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl RegularCardValue {
    fn numeric_value(&self) -> usize {
        *self as usize
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone, Sequence)]
enum RegularCardSuite {
    Heart,
    Diamond,
    Spade,
    Clubs,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone, Sequence)]
enum SpecialCardType {
    Dragon,
    Phoenix,
    One,
    Dog,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Sequence)]
enum Card {
    SpecialCard(SpecialCardType),
    RegularCard(RegularCardValue, RegularCardSuite),
}

impl Card {
    fn can_be_played_on_top_of_single_card(&self, other: &Card) -> bool {
        match (self, other) {
            (Card::RegularCard(self_value, _), Card::RegularCard(other_value, _)) => {
                self_value > other_value
            }

            (Card::SpecialCard(SpecialCardType::Dragon), _) => true,

            (Card::SpecialCard(SpecialCardType::Phoenix), Card::RegularCard(_, _)) => true,
            (Card::RegularCard(_, _), Card::SpecialCard(SpecialCardType::Phoenix)) => true,
            (
                Card::SpecialCard(SpecialCardType::Phoenix),
                Card::SpecialCard(SpecialCardType::One),
            ) => true,
            (
                Card::SpecialCard(SpecialCardType::Phoenix),
                Card::SpecialCard(SpecialCardType::Dog),
            ) => true,
            (
                Card::SpecialCard(SpecialCardType::Phoenix),
                Card::SpecialCard(SpecialCardType::Dragon),
            ) => false,

            (Card::SpecialCard(SpecialCardType::One), Card::SpecialCard(SpecialCardType::Dog)) => {
                true
            }
            (Card::SpecialCard(SpecialCardType::One), _) => false,

            (Card::SpecialCard(SpecialCardType::Dog), _) => false,

            // Any other combination is covered by the reverse. This works because special cards
            // can only occur once.
            (this_card @ Card::RegularCard(_, _), other_card) => {
                !other_card.can_be_played_on_top_of_single_card(this_card)
            }

            _ => panic!(),
        }
    }

    fn is_valid_in_multi_card_hand(&self) -> bool {
        match self {
            Card::RegularCard(_, _) => true,
            Card::SpecialCard(SpecialCardType::Phoenix) => true,
            Card::SpecialCard(SpecialCardType::One) => true,
            _ => false,
        }
    }

    fn value(&self) -> Option<RegularCardValue> {
        match self {
            Card::RegularCard(value, _) => Some(*value),
            _ => None,
        }
    }

    fn numeric_value(&self) -> Option<usize> {
        match self {
            Card::SpecialCard(SpecialCardType::One) => Some(1),
            Card::RegularCard(value, _) => Some(value.numeric_value()),
            _ => None,
        }
    }

    fn suite(&self) -> Option<RegularCardSuite> {
        match self {
            Card::RegularCard(_, suite) => Some(*suite),
            _ => None,
        }
    }

    fn score(&self) -> i8 {
        match self {
            Card::RegularCard(RegularCardValue::King, _) => 10,
            Card::RegularCard(RegularCardValue::Ten, _) => 10,
            Card::RegularCard(RegularCardValue::Five, _) => 5,
            Card::SpecialCard(SpecialCardType::Dragon) => 25,
            Card::SpecialCard(SpecialCardType::Phoenix) => -25,
            _ => 0,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum HandType {
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
struct Hand(Vec<Card>);

impl Hand {
    fn new(mut cards: Vec<Card>) -> Hand {
        cards.sort_unstable();
        Hand(cards)
    }

    fn hand_type(&self) -> Option<HandType> {
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

    fn is_valid_quadruple_bomb(&self) -> bool {
        match self.0.as_slice() {
            [Card::RegularCard(_, _), Card::RegularCard(_, _), Card::RegularCard(_, _), Card::RegularCard(_, _)] => {
                iter_all_equal(self.0.iter().map(|card| card.value())).is_some()
            }
            _ => false,
        }
    }

    fn is_valid_straight_bomb(&self) -> bool {
        self.is_valid_straight()
            && self.num_phoenices() == 0
            && iter_all_equal(self.0.iter().map(|card| card.suite())).is_some()
    }

    fn is_valid_single_card(&self) -> bool {
        self.0.len() == 1
    }

    fn is_valid_pair(&self) -> bool {
        match self.0.as_slice() {
            [Card::RegularCard(_, _), Card::RegularCard(_, _)] => {
                iter_all_equal(self.0.iter().map(|card| card.value())).is_some()
            }
            [Card::SpecialCard(SpecialCardType::Phoenix), Card::RegularCard(_, _)] => true,
            _ => false,
        }
    }

    fn is_valid_triple(&self) -> bool {
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

    fn is_valid_straight(&self) -> bool {
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

    fn is_valid_straight_of_pairs(&self) -> bool {
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
            .group_by(|card| card.numeric_value())
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

    fn is_valid_full_house(&self) -> bool {
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

    fn num_phoenices(&self) -> usize {
        self.0
            .iter()
            .filter(|card| match card {
                Card::SpecialCard(SpecialCardType::Phoenix) => true,
                _ => false,
            })
            .count()
    }

    fn relevant_card_value(&self) -> Option<RegularCardValue> {
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

    fn is_bomb(&self) -> bool {
        match self.hand_type() {
            Some(HandType::StraightBomb) | Some(HandType::QuadrupleBomb) => true,
            _ => false,
        }
    }

    fn higher_value_than(&self, other: &Hand) -> bool {
        self.relevant_card_value()
            .zip(other.relevant_card_value())
            .map(|(self_value, other_value)| self_value > other_value)
            .unwrap_or(false)
    }

    fn can_be_played_on(&self, other: &Hand) -> bool {
        if self.is_bomb() {
            if other.is_bomb() {
                self.0.len() > other.0.len()
                    || (self.0.len() == other.0.len() && self.higher_value_than(other))
            } else {
                true
            }
        } else {
            self.0.len() == other.0.len() && self.higher_value_than(other)
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct PlayerHand(Vec<Card>);

#[derive(PartialEq, Eq, Debug)]
struct Deck(Vec<Card>);

const NUM_CARDS_PER_PLAYER: usize = 14;
const NUM_PLAYERS: usize = 4;

impl Deck {
    fn new<R: rand::RngCore>(rng: &mut R) -> Deck {
        let mut cards = all::<Card>().collect::<Vec<_>>();
        cards.as_mut_slice().shuffle(rng);
        Deck(cards)
    }

    fn deal(&mut self) -> PlayerHand {
        let mut cards = self.0.split_off(self.0.len() - NUM_CARDS_PER_PLAYER);
        cards.sort_unstable();
        PlayerHand(cards)
    }
}

fn print_hand(hand: &Hand) {
    println!(
        "hand {:?} has type {:?} at relevant value {:?}",
        hand,
        hand.hand_type(),
        hand.relevant_card_value()
    );
}

fn main() {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(10);
    let mut deck = Deck::new(&mut rng);
    for i in 0..NUM_PLAYERS {
        println!("Player {} is dealt hand {:?}", i, deck.deal());
    }
    let card1 = Card::RegularCard(RegularCardValue::King, RegularCardSuite::Heart);
    let card2 = Card::RegularCard(RegularCardValue::Four, RegularCardSuite::Clubs);
    let card_one = Card::SpecialCard(SpecialCardType::One);
    println!(
        "{:?} can be played on top of {:?}: {:?}",
        card1,
        card2,
        card1.can_be_played_on_top_of_single_card(&card2)
    );
    println!(
        "{:?} can be played on top of {:?}: {:?}",
        card_one,
        card2,
        card_one.can_be_played_on_top_of_single_card(&card2)
    );
    println!(
        "{:?} can be played on top of {:?}: {:?}",
        card2,
        card_one,
        card2.can_be_played_on_top_of_single_card(&card_one)
    );
    println!("{:?} scores {:?} points", card1, card1.score());

    let hand = Hand(vec![card1]);
    print_hand(&hand);

    let hand2 = Hand::new(vec![
        Card::RegularCard(RegularCardValue::Two, RegularCardSuite::Heart),
        Card::RegularCard(RegularCardValue::Three, RegularCardSuite::Heart),
        Card::RegularCard(RegularCardValue::Four, RegularCardSuite::Heart),
        Card::RegularCard(RegularCardValue::Five, RegularCardSuite::Heart),
        Card::RegularCard(RegularCardValue::Six, RegularCardSuite::Heart),
    ]);
    print_hand(&hand2);

    let hand3 = Hand::new(vec![
        Card::RegularCard(RegularCardValue::Two, RegularCardSuite::Heart),
        Card::RegularCard(RegularCardValue::Three, RegularCardSuite::Heart),
        Card::RegularCard(RegularCardValue::Four, RegularCardSuite::Heart),
        Card::RegularCard(RegularCardValue::Five, RegularCardSuite::Heart),
        Card::RegularCard(RegularCardValue::Seven, RegularCardSuite::Heart),
    ]);
    print_hand(&hand3);

    let hand4 = Hand::new(vec![
        Card::RegularCard(RegularCardValue::Two, RegularCardSuite::Heart),
        Card::RegularCard(RegularCardValue::Three, RegularCardSuite::Heart),
        Card::RegularCard(RegularCardValue::Four, RegularCardSuite::Heart),
        Card::RegularCard(RegularCardValue::Six, RegularCardSuite::Heart),
        Card::SpecialCard(SpecialCardType::Phoenix),
    ]);
    print_hand(&hand4);

    let hand5 = Hand::new(vec![
        Card::RegularCard(RegularCardValue::Two, RegularCardSuite::Heart),
        Card::RegularCard(RegularCardValue::Three, RegularCardSuite::Heart),
        Card::RegularCard(RegularCardValue::Four, RegularCardSuite::Heart),
        Card::RegularCard(RegularCardValue::Five, RegularCardSuite::Heart),
        Card::SpecialCard(SpecialCardType::One),
    ]);
    print_hand(&hand5);

    let hand6 = Hand::new(vec![
        Card::RegularCard(RegularCardValue::Two, RegularCardSuite::Heart),
        Card::RegularCard(RegularCardValue::Two, RegularCardSuite::Clubs),
        Card::RegularCard(RegularCardValue::Two, RegularCardSuite::Clubs),
        Card::RegularCard(RegularCardValue::Four, RegularCardSuite::Diamond),
        Card::RegularCard(RegularCardValue::Four, RegularCardSuite::Spade),
    ]);
    print_hand(&hand6);

    let hand7 = Hand::new(vec![
        Card::RegularCard(RegularCardValue::Two, RegularCardSuite::Heart),
        Card::RegularCard(RegularCardValue::Two, RegularCardSuite::Clubs),
        Card::RegularCard(RegularCardValue::Three, RegularCardSuite::Clubs),
        Card::RegularCard(RegularCardValue::Three, RegularCardSuite::Diamond),
        Card::RegularCard(RegularCardValue::Four, RegularCardSuite::Spade),
        Card::RegularCard(RegularCardValue::Four, RegularCardSuite::Heart),
    ]);
    print_hand(&hand7);

    let hand8 = Hand::new(vec![
        Card::RegularCard(RegularCardValue::Two, RegularCardSuite::Heart),
        Card::RegularCard(RegularCardValue::Two, RegularCardSuite::Clubs),
        Card::RegularCard(RegularCardValue::Two, RegularCardSuite::Diamond),
        Card::RegularCard(RegularCardValue::Two, RegularCardSuite::Spade),
    ]);
    print_hand(&hand8);
}
