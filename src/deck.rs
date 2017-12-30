use rand::StdRng;
use rand::Rng;

use cards::{Suit, Rank, Card};


#[derive(Debug, Clone)]
pub struct Deck {
    pub cards: [Card; 32],
    randomness: StdRng,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = [Card::new(Suit::Spades, Rank::Ace); 32];

        let mut i = 0;
        for suit in Suit::iterator() {
            for rank in Rank::iterator() {
                cards[i] = Card::new(*suit, *rank);
                i += 1;
            }
        }

        let mut deck = Deck {
            cards,
            randomness: StdRng::new().unwrap(),
        };
        // Always return a shuffled deck
        deck.shuffle();
        deck
    }

    /// Shuffle the cards in the deck in-place
    pub fn shuffle(&mut self) {
        self.randomness.shuffle(&mut self.cards);
    }

    /// Deal the cards to all players, essentially 4 arrays of 8 cards
    /// Dealing in belote is done in a 3-2-3 way
    pub fn deal(&self) -> [Vec<Card>; 4] {
        let mut cards1 = vec![];
        let mut cards2 = vec![];
        let mut cards3 = vec![];
        let mut cards4 = vec![];

        // index in the deck
        let mut j = 0;
        for i in 0..3 {
            let num_cards = if i == 1 { 2 } else { 3 };
            cards1.extend(self.cards[j..j+num_cards].iter().cloned());
            j += num_cards;
            cards2.extend(self.cards[j..j+num_cards].iter().cloned());
            j += num_cards;
            cards3.extend(self.cards[j..j+num_cards].iter().cloned());
            j += num_cards;
            cards4.extend(self.cards[j..j+num_cards].iter().cloned());
            j += num_cards;
        }

        [cards1, cards2, cards3, cards4]
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn can_create_and_shuffle_deck() {
        let deck = Deck::new();
        let deck2 = Deck::new();
        assert_ne!(deck.cards, deck2.cards);
    }

    #[test]
    fn can_deal_cards() {
        let deck = Deck::new();
        let hands = deck.deal();
        assert_eq!(hands.len(), 4);

        assert_eq!(hands[0].len(), 8);
        assert_eq!(hands[1].len(), 8);
        assert_eq!(hands[2].len(), 8);
        assert_eq!(hands[3].len(), 8);

        // no cards duplicate
        let mut cards = HashMap::new();
        for c in &hands[0] {
            assert!(!cards.contains_key(c));
            cards.insert(c, true);
        }
        for c in &hands[1] {
            assert!(!cards.contains_key(c));
            cards.insert(c, true);
        }
        for c in &hands[2] {
            assert!(!cards.contains_key(c));
            cards.insert(c, true);
        }
        for c in &hands[3] {
            assert!(!cards.contains_key(c));
            cards.insert(c, true);
        }
    }
}
