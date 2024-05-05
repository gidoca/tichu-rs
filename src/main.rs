use enum_iterator::{all, Sequence};
use rand::{seq::SliceRandom, SeedableRng};

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

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone, Sequence)]
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

    fn numeric_value(&self) -> Option<usize> {
        match self {
            Card::SpecialCard(SpecialCardType::One) => Some(1),
            Card::RegularCard(value, _) => Some(value.numeric_value()),
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

    let hand5 = Hand::new(vec![
        Card::RegularCard(RegularCardValue::Two, RegularCardSuite::Heart),
        Card::RegularCard(RegularCardValue::Three, RegularCardSuite::Heart),
        Card::RegularCard(RegularCardValue::Four, RegularCardSuite::Heart),
        Card::RegularCard(RegularCardValue::Five, RegularCardSuite::Heart),
        Card::SpecialCard(SpecialCardType::One),
    ]);
    println!("hand {:?} has type {:?}", hand5, hand5.hand_type());
}
