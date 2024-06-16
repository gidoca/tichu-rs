mod card;
mod hand;
mod util;

use enum_iterator::all;

use rand::{seq::SliceRandom, SeedableRng};

use card::*;
use hand::*;

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

    let hand = Hand::single_card(card1);
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
