#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
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

#[derive(PartialEq, Eq, Debug)]
enum RegularCardSuite {
    Heart,
    Diamond,
    Spade,
    Clubs,
}

#[derive(PartialEq, Eq, Debug)]
enum SpecialCardType {
    Dragon,
    Phoenix,
    One,
    Dog,
}

#[derive(PartialEq, Eq, Debug)]
enum Card {
    RegularCard(RegularCardSuite, RegularCardValue),
    SpecialCard(SpecialCardType),
}

impl Card {
    fn is_higher_single_card_than(&self, other: &Card) -> bool {
        match (self, other) {
            (Card::RegularCard(_, self_value), Card::RegularCard(_, other_value)) => self_value > other_value,
            _ => panic!()
        }
    }
}

fn main() {
    let card1 = Card::RegularCard(RegularCardSuite::Heart, RegularCardValue::Ace);
    let card2 = Card::RegularCard(RegularCardSuite::Clubs, RegularCardValue::Four);
    println!("Ace larger than Four: {:?}", card1.is_higher_single_card_than(&card2));
}
