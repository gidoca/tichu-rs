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

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum RegularCardSuite {
    Heart,
    Diamond,
    Spade,
    Clubs,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum SpecialCardType {
    Dragon,
    Phoenix,
    One,
    Dog,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Card {
    RegularCard(RegularCardSuite, RegularCardValue),
    SpecialCard(SpecialCardType),
}

impl Card {
    fn can_be_played_on_top_of_single_card(&self, other: &Card) -> bool {
        match (self, other) {
            (Card::RegularCard(_, self_value), Card::RegularCard(_, other_value)) => self_value > other_value,

            (Card::SpecialCard(SpecialCardType::Dragon), _) => true,

            (Card::SpecialCard(SpecialCardType::Phoenix), Card::RegularCard(_, _)) => true,
            (Card::RegularCard(_, _), Card::SpecialCard(SpecialCardType::Phoenix)) => true,
            (Card::SpecialCard(SpecialCardType::Phoenix), Card::SpecialCard(SpecialCardType::One)) => true,
            (Card::SpecialCard(SpecialCardType::Phoenix), Card::SpecialCard(SpecialCardType::Dog)) => true,
            (Card::SpecialCard(SpecialCardType::Phoenix), Card::SpecialCard(SpecialCardType::Dragon)) => false,

            (Card::SpecialCard(SpecialCardType::One), Card::SpecialCard(SpecialCardType::Dog)) => true,
            (Card::SpecialCard(SpecialCardType::One), _) => false,

            (Card::SpecialCard(SpecialCardType::Dog), _) => false,

            // Any other combination is covered by the reverse. This works because special cards
            // can only occur once.
            (Card::RegularCard(color, value), other_card) => !other_card.can_be_played_on_top_of_single_card(&Card::RegularCard(*color, *value)),

            _ => panic!()
        }
    }

    fn score(&self) -> i8 {
        match(self) {
            Card::RegularCard(_, RegularCardValue::King) => 10,
            Card::RegularCard(_, RegularCardValue::Ten) => 10,
            Card::RegularCard(_, RegularCardValue::Five) => 5,
            Card::SpecialCard(SpecialCardType::Dragon) => 25,
            Card::SpecialCard(SpecialCardType::Phoenix) => -25,
            _ => 0,
        }
    }
}

fn main() {
    let card1 = Card::RegularCard(RegularCardSuite::Heart, RegularCardValue::Ace);
    let card2 = Card::RegularCard(RegularCardSuite::Clubs, RegularCardValue::Four);
    println!("{:?} can be played on top of {:?}: {:?}", card1, card2, card1.can_be_played_on_top_of_single_card(&card2));
}
