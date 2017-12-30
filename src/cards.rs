use std::fmt;
use std::slice::Iter;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Suit {
    // We cannot iterate on enum values in Rust so we duplicate a bit the code
    // here to be able to iterate on the values in the deck
    pub fn iterator() -> Iter<'static, Suit> {
        [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades].into_iter()
    }
}

/// Belote is played with 32 cards, from 7 to Ace
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Rank {
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    // Same as in Suit
    pub fn iterator() -> Iter<'static, Rank> {
        [
            Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten,
            Rank::Jack, Rank::Queen, Rank::King, Rank::Ace,
        ].into_iter()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Card {
        Card {
            suit,
            rank,
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} of {:?}", self.rank, self.suit)
    }
}
