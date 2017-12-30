use std::collections::HashMap;

use cards::Suit;
use bids::Contract;
use players::{Player, Team};


/// A round of the actual game, after a contract has been established
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Round {
    contract: Contract,
    pub scores: HashMap<Team, usize>,
    //hands: Vec<Vec<Cards>>,
}

impl Round {
    fn new(contract: Contract) -> Round {
        let mut scores = HashMap::new();
        scores.insert(Team::SouthNorth, 0);
        scores.insert(Team::EastWest, 0);

        Round {
            contract,
            scores,
        }
    }

    /// Calculates the points for each team according to the contract
    fn calculate_points(&mut self) {

    }
}
