use enum_iterator::Sequence;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone, Sequence)]
pub enum RegularCardValue {
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
    pub fn numeric_value(&self) -> usize {
        *self as usize
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone, Sequence)]
pub enum RegularCardSuite {
    Heart,
    Diamond,
    Spade,
    Clubs,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone, Sequence)]
pub enum SpecialCardType {
    Dragon,
    Phoenix,
    One,
    Dog,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Sequence)]
pub enum Card {
    SpecialCard(SpecialCardType),
    RegularCard(RegularCardValue, RegularCardSuite),
}

impl Card {
    pub fn can_be_played_on_top_of_single_card(&self, other: &Card) -> bool {
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

    pub fn is_valid_in_multi_card_hand(&self) -> bool {
        match self {
            Card::RegularCard(_, _) => true,
            Card::SpecialCard(SpecialCardType::Phoenix) => true,
            Card::SpecialCard(SpecialCardType::One) => true,
            _ => false,
        }
    }

    pub fn value(&self) -> Option<RegularCardValue> {
        match self {
            Card::RegularCard(value, _) => Some(*value),
            _ => None,
        }
    }

    pub fn numeric_value(&self) -> Option<usize> {
        match self {
            Card::SpecialCard(SpecialCardType::One) => Some(1),
            Card::RegularCard(value, _) => Some(value.numeric_value()),
            _ => None,
        }
    }

    pub fn suite(&self) -> Option<RegularCardSuite> {
        match self {
            Card::RegularCard(_, suite) => Some(*suite),
            _ => None,
        }
    }

    pub fn score(&self) -> i8 {
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
