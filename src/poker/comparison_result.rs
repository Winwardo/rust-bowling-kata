use super::card_rank::*;

#[derive(Debug, PartialEq)]
pub enum Players {
    Black,
    White,
}

impl Players {
    pub fn to_string(&self) -> &str {
        match *self {
            Players::Black => "Black",
            Players::White => "White",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WinType {
    HighCard(CardRank),
    Pair(CardRank),
    PairHighCard(CardRank),
}

#[derive(Debug, PartialEq)]
pub struct Winner {
    pub player: Players,
    pub win_type: WinType,
}

#[derive(Debug, PartialEq)]
pub struct ComparisonResult {
    pub winner: Option<Winner>,
    _s: (),
}

impl ComparisonResult {
    pub fn from_winner(winner: Winner) -> ComparisonResult {
        ComparisonResult {
            winner: Some(winner),
            _s: (),
        }
    }

    pub fn tie() -> ComparisonResult {
        ComparisonResult {
            winner: None,
            _s: (),
        }
    }
}
