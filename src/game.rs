use deck::Deck;
use cards::Suit;
use bids::BidPhase;
use players::{Player, Team};
use round::Round;

static SCORE_GOAL: usize = 1000;



#[derive(Debug, Clone)]
pub struct Game {
    /// Which player is starting the current round.
    /// Moves clockwise by one at the end of a round
    /// south -> west -> north -> east -> south
    first_player: Player,
    /// All the rounds in the current game
    /// Resets when a team reaches SCORE_GOAL
    rounds: Vec<Round>,
    /// The deck the game is going to use
    deck: Deck,
}

impl Game {
    pub fn new() -> Game {
        Game {
            first_player: Player::South,
            rounds: Vec::new(),
            deck: Deck::new(),
        }
    }

    fn is_initial_round(&self) -> bool {
        self.rounds.is_empty()
    }

    pub fn new_round(&mut self) {
        // move to next player except on the first round
        if !self.is_initial_round() {
            self.first_player = self.first_player.next_player();
        }
        self.deck.shuffle();
        let cards = self.deck.deal();
    }

    /// Returns the winner team if there is one
    pub fn has_winner(&self) -> Option<Team> {
        let mut sn_score = 0;
        let mut ew_score = 0;
        for round in &self.rounds {
            sn_score += round.scores[&Team::SouthNorth];
            ew_score += round.scores[&Team::EastWest];
        }
        if sn_score > SCORE_GOAL && sn_score > ew_score {
            Some(Team::SouthNorth)
        } else if ew_score > SCORE_GOAL {
            Some(Team::EastWest)
        } else {
            None
        }
    }
}

