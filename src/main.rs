#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
enum RegularCardValue {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
}

impl RegularCardValue {
    fn numeric_value(&self) -> usize {
        *self as usize
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
enum RegularCardSuite {
    Heart,
    Diamond,
    Spade,
    Clubs,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
enum SpecialCardType {
    Dragon,
    Phoenix,
    One,
    Dog,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
enum Card {
    RegularCard(RegularCardValue, RegularCardSuite),
    SpecialCard(SpecialCardType),
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
            hand if hand.is_valid_single_card() => Some(HandType::SingleCard),
            hand if hand.is_valid_pair() => Some(HandType::Pair),
            hand if hand.is_valid_triple() => Some(HandType::Triple),
            hand if hand.is_valid_straight() => Some(HandType::Straight),
            _ => None,
        }
    }

    fn is_valid_single_card(&self) -> bool {
        self.0.len() == 1
    }

    fn is_valid_pair(&self) -> bool {
        match self.0.as_slice() {
            [Card::RegularCard(value1, _), Card::RegularCard(value2, _)] if value1 == value2 => {
                true
            }
            [Card::RegularCard(_, _), Card::SpecialCard(SpecialCardType::Phoenix)] => true,
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
            [Card::RegularCard(value1, _), Card::RegularCard(value2, _), Card::SpecialCard(SpecialCardType::Phoenix)]
                if value1 == value2 =>
            {
                true
            }
            _ => false,
        }
    }

    fn is_valid_straight(&self) -> bool {
        let num_phoenices = self
            .0
            .iter()
            .filter(|card| match card {
                Card::SpecialCard(SpecialCardType::Phoenix) => true,
                _ => false,
            })
            .count();
        let num_phoenices_needed = self
            .0
            .as_slice()
            .windows(2)
            .filter_map(|pair| match pair {
                [Card::RegularCard(value1, _), Card::RegularCard(value2, _)] => {
                    Some(value2.numeric_value() - value1.numeric_value() - 1)
                }
                _ => None,
            })
            .sum();
        self.0.len() >= 5 && num_phoenices >= num_phoenices_needed
    }
}

fn main() {
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
    println!("hand {:?} has type {:?}", hand, hand.hand_type());

    let hand2 = Hand::new(vec![
        Card::RegularCard(RegularCardValue::Two, RegularCardSuite::Heart),
        Card::RegularCard(RegularCardValue::Three, RegularCardSuite::Heart),
        Card::RegularCard(RegularCardValue::Four, RegularCardSuite::Heart),
        Card::RegularCard(RegularCardValue::Five, RegularCardSuite::Heart),
        Card::RegularCard(RegularCardValue::Six, RegularCardSuite::Heart),
    ]);
    println!("hand {:?} has type {:?}", hand2, hand2.hand_type());

    let hand3 = Hand::new(vec![
        Card::RegularCard(RegularCardValue::Two, RegularCardSuite::Heart),
        Card::RegularCard(RegularCardValue::Three, RegularCardSuite::Heart),
        Card::RegularCard(RegularCardValue::Four, RegularCardSuite::Heart),
        Card::RegularCard(RegularCardValue::Five, RegularCardSuite::Heart),
        Card::RegularCard(RegularCardValue::Seven, RegularCardSuite::Heart),
    ]);
    println!("hand {:?} has type {:?}", hand3, hand3.hand_type());

    let hand4 = Hand::new(vec![
        Card::RegularCard(RegularCardValue::Two, RegularCardSuite::Heart),
        Card::RegularCard(RegularCardValue::Three, RegularCardSuite::Heart),
        Card::RegularCard(RegularCardValue::Four, RegularCardSuite::Heart),
        Card::RegularCard(RegularCardValue::Six, RegularCardSuite::Heart),
        Card::SpecialCard(SpecialCardType::Phoenix),
    ]);
    println!("hand {:?} has type {:?}", hand4, hand4.hand_type());
}
