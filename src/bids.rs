use std::slice::Iter;

use failure::Error;

use cards::Suit;
use players::Player;


#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, PartialOrd, Ord)]
pub enum Bid {
    Pass,
    Eighty,
    Ninety,
    Hundred,
    HundredTen,
    HundredTwenty,
    HundredThirty,
    HundredForty,
    HundredFifty,
    HundredSixty,
    // The opponent team doesn't win a single round
    Capot,
    Counter,
    DoubleCounter,
}

impl Bid {
    // We cannot iterate on enum values in Rust so we duplicate a bit the code
    // here to be able to iterate on the values in the deck
    pub fn iterator() -> Iter<'static, Bid> {
        [
            Bid::Pass,
            Bid::Eighty,
            Bid::Ninety,
            Bid::Hundred,
            Bid::HundredTen,
            Bid::HundredTwenty,
            Bid::HundredThirty,
            Bid::HundredForty,
            Bid::HundredFifty,
            Bid::HundredSixty,
            Bid::Capot,
            Bid::Counter,
            Bid::DoubleCounter,
        ].into_iter()
    }

    pub fn requires_suit(&self) -> bool {
        match *self {
            Bid::Pass | Bid::Counter | Bid::DoubleCounter => false,
            _ => true
        }
    }
}

/// Which state of the bidding phase are we at
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, PartialOrd, Ord)]
pub enum BiddingState {
    /// Players can bid
    Ongoing,
    /// All players passed their turn without bidding: deal cards again
    DealAgain,
    /// A bid has been done and the game should start
    Done
}

/// The bid that won the bidding phase and whether it has been countered/double countered
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Contract {
    player: Player,
    bid: Bid,
    suit: Suit,
    countered: bool,
    double_countered: bool,
}

impl Contract {
    fn new(bids: &[(Player, Bid, Option<Suit>)]) -> Result<Contract, Error> {
        let mut countered = false;
        let mut double_countered = false;

        for &(player, bid, suit) in bids.iter().rev() {
            if bid == Bid::DoubleCounter {
                double_countered = true;
                continue;
            }
            if bid == Bid::Counter {
                countered = true;
                continue;
            }
            if bid != Bid::Pass {
                return Ok(Contract {player, bid, suit: suit.unwrap(), countered, double_countered});
            }
        }

        bail!("Couldn't get a contract from the list of bids")
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BidPhase {
    /// Who is starting the bid phase
    starting_player: Player,
    /// All the valid submitted bids.
    /// Pass has no suit associated with it but all the others have
    bids: Vec<(Player, Bid, Option<Suit>)>,
    /// Whether the current bid has been countered by the opposing team
    countered: Option<Player>,
    pub state: BiddingState,
}


impl BidPhase {
    pub fn new(starting_player: Player) -> BidPhase {
        BidPhase {
            starting_player,
            bids: vec![],
            countered: None,
            state: BiddingState::Ongoing,
        }
    }

    /// Finds all available bids for the given player
    pub fn available_bids(&self, player: Player) -> Vec<Bid> {
        // everything allowed except counter/double counter
        if self.last_bid().is_none() {
            return Bid::iterator()
                .filter(|b| **b != Bid::Counter && **b != Bid::DoubleCounter)
                .cloned()
                .collect();
        }

        // we already checked before if we had one
        let last_bid = self.last_bid().unwrap();
        if last_bid.1 == Bid::DoubleCounter {
            // Game should start now
            return vec![];
        }

        // Doing the counter first
        if last_bid.1 == Bid::Counter {
            // Same team as counter: can only pass
            if last_bid.0.team() == player.team() {
                return vec![Bid::Pass];
            } else {
                // different team as counter: can pass and double counter
                return vec![Bid::Pass, Bid::DoubleCounter];
            }
        }

        // Back to normal bids now
        // Pass is always allowed
        let mut bids = vec![Bid::Pass];
        let all_bids: Vec<Bid> = Bid::iterator()
            .filter(|b| **b > last_bid.1 && **b != Bid::DoubleCounter)
            .cloned()
            .collect();

        bids.extend(all_bids);
        bids
    }

    /// Update the state of the bidding phase
    fn next_state(&self) -> BiddingState {
        // Bid phase can only be over if there are at least 4 bids
        if self.bids.len() <= 3 {
            return BiddingState::Ongoing;
        }

        let mut pass_count = 0;
        for &(_, bid, _) in self.bids.iter().rev() {
            if bid == Bid::Pass {
                pass_count += 1;
            }
            if self.last_bid().is_some() && pass_count == 3 {
                // We have a real bid and everyone else passed
                return BiddingState::Done;
            }
            if pass_count == 4 {
                // Everyone passed without any bid
                return BiddingState::DealAgain;
            }
            // we have a bid still going on, no need to look further
            if bid != Bid::Pass {
                break;
            }
        }

        BiddingState::Ongoing
    }

    /// The last actual (= other than pass) bid.
    pub fn last_bid(&self) -> Option<(Player, Bid)> {
        if self.bids.is_empty() {
            return None;
        }

        for &b in self.bids.iter().rev() {
            if b.1 != Bid::Pass {
                return Some((b.0, b.1));
            }
        }

        None
    }

    /// Add a bid if possible and returns an error if an invalid bid was submitted.
    pub fn bid(&mut self, player: Player, bid: Bid, suit: Option<Suit>) -> Result<(), Error> {
        if self.state != BiddingState::Ongoing {
            bail!("The bidding phase is over!");
        }

        // Is a player trying to be sneaky and skip the order?
        if let Some(&(last_player, _, _)) = self.bids.last() {
            if last_player.next_player() != player {
                bail!("Wrong player");
            }
        } else {
            if player != self.starting_player {
                bail!("Wrong player");
            }
        }

        // First bid: everything is okay
        if self.bids.is_empty() {
            self.bids.push((player, bid, suit));
            // no need to update state, still ongoing
            return Ok(());
        }

        if !self.available_bids(player).contains(&bid) {
            bail!("Bid not possible");
        }

        if suit.is_none() && bid.requires_suit() {
            bail!("A bid other than pass/counter/doublecounter must have a suit associated");
        }
        self.bids.push((player, bid, suit));

        self.state = self.next_state();
        Ok(())
    }

    pub fn get_contract(&self) -> Result<Contract, Error> {
        if self.state != BiddingState::Done {
            bail!("Invalid bidding state: expected the bidding phase to be done with a bid");
        }

        Contract::new(&self.bids)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_find_all_initial_possible_bids() {
        let bid_phase = BidPhase::new(Player::South);
        let available_bids = bid_phase.available_bids(Player::South);
        assert_eq!(
            available_bids,
            vec![
                Bid::Pass,
                Bid::Eighty, Bid::Ninety, Bid::Hundred,
                Bid::HundredTen, Bid::HundredTwenty, Bid::HundredThirty,
                Bid::HundredForty, Bid::HundredFifty, Bid::HundredSixty,
                Bid::Capot,
            ]
        );
    }

    #[test]
    fn can_find_available_bids_after_bid() {
        let mut bid_phase = BidPhase::new(Player::South);
        assert!(bid_phase.bid(Player::South, Bid::HundredTwenty, Some(Suit::Hearts)).is_ok());
        let available_bids = bid_phase.available_bids(Player::West);
        assert_eq!(
            available_bids,
            vec![
                Bid::Pass, Bid::HundredThirty, Bid::HundredForty,
                Bid::HundredFifty, Bid::HundredSixty, Bid::Capot,
                Bid::Counter,
            ]
        );
    }

    #[test]
    fn can_find_available_bids_after_counter_other_team() {
        let mut bid_phase = BidPhase::new(Player::South);
        assert!(bid_phase.bid(Player::South, Bid::HundredTwenty, Some(Suit::Hearts)).is_ok());
        assert!(bid_phase.bid(Player::West, Bid::Counter, None).is_ok());
        let available_bids = bid_phase.available_bids(Player::North);
        assert_eq!(
            available_bids,
            vec![Bid::Pass, Bid::DoubleCounter]
        );
    }

    #[test]
    fn can_find_available_bids_after_counter_same_team() {
        let mut bid_phase = BidPhase::new(Player::South);
        assert!(bid_phase.bid(Player::South, Bid::HundredTwenty, Some(Suit::Hearts)).is_ok());
        assert!(bid_phase.bid(Player::West, Bid::Counter, None).is_ok());
        let available_bids = bid_phase.available_bids(Player::East);
        assert_eq!(
            available_bids,
            vec![Bid::Pass]
        );
    }

    #[test]
    fn errors_on_invalid_player_bid() {
        let mut bid_phase = BidPhase::new(Player::South);
        assert!(bid_phase.bid(Player::North, Bid::Capot, None).is_err());
    }

    #[test]
    fn players_can_all_pass() {
        let mut bid_phase = BidPhase::new(Player::South);
        assert!(bid_phase.bid(Player::South, Bid::Pass, None).is_ok());
        assert!(bid_phase.bid(Player::West, Bid::Pass, None).is_ok());
        assert!(bid_phase.bid(Player::North, Bid::Pass, None).is_ok());
        assert!(bid_phase.bid(Player::East, Bid::Pass, None).is_ok());
        assert_eq!(bid_phase.state, BiddingState::DealAgain);
    }

    #[test]
    fn a_simple_bid_phase_can_be_over() {
        let mut bid_phase = BidPhase::new(Player::South);
        assert!(bid_phase.bid(Player::South, Bid::Eighty, Some(Suit::Spades)).is_ok());
        assert!(bid_phase.bid(Player::West, Bid::Pass, None).is_ok());
        assert!(bid_phase.bid(Player::North, Bid::Pass, None).is_ok());
        assert!(bid_phase.bid(Player::East, Bid::Pass, None).is_ok());
        assert_eq!(bid_phase.state, BiddingState::Done);
        let contract = bid_phase.get_contract().unwrap();
        assert_eq!(contract.player, Player::South);
        assert_eq!(contract.bid, Bid::Eighty);
        assert_eq!(contract.suit, Suit::Spades);
        assert_eq!(contract.countered, false);
        assert_eq!(contract.double_countered, false);
    }

    #[test]
    fn another_simple_bid_phase_can_be_over() {
        let mut bid_phase = BidPhase::new(Player::South);
        assert!(bid_phase.bid(Player::South, Bid::Pass, None).is_ok());
        assert!(bid_phase.bid(Player::West, Bid::Eighty, Some(Suit::Spades)).is_ok());
        assert!(bid_phase.bid(Player::North, Bid::Pass, None).is_ok());
        assert!(bid_phase.bid(Player::East, Bid::Pass, None).is_ok());
        assert!(bid_phase.bid(Player::South, Bid::Pass, None).is_ok());
        assert_eq!(bid_phase.state, BiddingState::Done);
        let contract = bid_phase.get_contract().unwrap();
        assert_eq!(contract.player, Player::West);
        assert_eq!(contract.bid, Bid::Eighty);
        assert_eq!(contract.suit, Suit::Spades);
        assert_eq!(contract.countered, false);
        assert_eq!(contract.double_countered, false);
    }

    #[test]
    fn a_complex_bid_phase_can_be_over() {
        let mut bid_phase = BidPhase::new(Player::South);
        assert!(bid_phase.bid(Player::South, Bid::Eighty, Some(Suit::Spades)).is_ok());
        assert!(bid_phase.bid(Player::West, Bid::Pass, None).is_ok());
        assert!(bid_phase.bid(Player::North, Bid::Ninety, Some(Suit::Spades)).is_ok());
        assert!(bid_phase.bid(Player::East, Bid::HundredTen, Some(Suit::Hearts)).is_ok());
        assert!(bid_phase.bid(Player::South, Bid::Pass, None).is_ok());
        assert!(bid_phase.bid(Player::West, Bid::HundredTwenty, Some(Suit::Hearts)).is_ok());
        assert!(bid_phase.bid(Player::North, Bid::Pass, None).is_ok());
        assert!(bid_phase.bid(Player::East, Bid::Pass, None).is_ok());
        assert_eq!(bid_phase.state, BiddingState::Ongoing);
        assert!(bid_phase.bid(Player::South, Bid::Pass, None).is_ok());
        assert_eq!(bid_phase.state, BiddingState::Done);

        let contract = bid_phase.get_contract().unwrap();
        assert_eq!(contract.player, Player::West);
        assert_eq!(contract.bid, Bid::HundredTwenty);
        assert_eq!(contract.suit, Suit::Hearts);
        assert_eq!(contract.countered, false);
        assert_eq!(contract.double_countered, false);
    }

    #[test]
    fn can_counter_opponent() {
        let mut bid_phase = BidPhase::new(Player::South);
        assert!(bid_phase.bid(Player::South, Bid::HundredTwenty, Some(Suit::Spades)).is_ok());
        assert!(bid_phase.bid(Player::West, Bid::Pass, None).is_ok());
        assert!(bid_phase.bid(Player::North, Bid::Pass, None).is_ok());
        assert!(bid_phase.bid(Player::East, Bid::Counter, None).is_ok());
        assert_eq!(bid_phase.state, BiddingState::Ongoing);
        let available_bids = bid_phase.available_bids(Player::South);
        assert_eq!(
            available_bids,
            vec![Bid::Pass, Bid::DoubleCounter]
        );
        assert!(bid_phase.bid(Player::South, Bid::Pass, None).is_ok());
        assert!(bid_phase.bid(Player::West, Bid::Pass, None).is_ok());
        assert!(bid_phase.bid(Player::North, Bid::Pass, None).is_ok());
        let contract = bid_phase.get_contract().unwrap();
        assert_eq!(contract.player, Player::South);
        assert_eq!(contract.bid, Bid::HundredTwenty);
        assert_eq!(contract.suit, Suit::Spades);
        assert_eq!(contract.countered, true);
        assert_eq!(contract.double_countered, false);
    }
}
