#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum Player {
    North,
    West,
    South,
    East,
}

impl Player {
    pub fn next_player(&self) -> Player {
        match *self {
            Player::South => Player::West,
            Player::West => Player::North,
            Player::North => Player::East,
            Player::East => Player::South,
        }
    }

    pub fn team(&self) -> Team {
        match *self {
            Player::South | Player::North => Team::SouthNorth,
            Player::East | Player::West => Team::EastWest,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum Team {
    SouthNorth,
    EastWest,
}
