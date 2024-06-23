use crate::card::Card;

use enum_iterator::all;

use rand::seq::SliceRandom;

#[derive(PartialEq, Eq, Debug)]
pub struct PlayerHand(Vec<Card>);

#[derive(PartialEq, Eq, Debug)]
pub struct Deck(Vec<Card>);

pub const NUM_CARDS_PER_PLAYER: usize = 14;
pub const NUM_PLAYERS: usize = 4;

impl Deck {
    pub fn new<R: rand::RngCore>(rng: &mut R) -> Deck {
        let mut cards = all::<Card>().collect::<Vec<_>>();
        cards.as_mut_slice().shuffle(rng);
        Deck(cards)
    }

    pub fn deal(&mut self) -> PlayerHand {
        let mut cards = self.0.split_off(self.0.len() - NUM_CARDS_PER_PLAYER);
        cards.sort_unstable();
        PlayerHand(cards)
    }
}

