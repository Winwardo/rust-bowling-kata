use super::card_rank::*;
use super::suits::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Card {
    pub rank: CardRank,
    pub suit: Suit,
    _s: (),
}

#[derive(Debug, PartialEq)]
pub enum CardError {
    InvalidSuit(char),
    InvalidRank(char),
    WrongLength,
}

impl Card {
    pub fn from_string(input: &str) -> Result<Card, CardError> {
        let mut chars = input.chars();
        let count = input.chars().count();

        if count == 2 {
            let rank_char = chars.next().unwrap();
            let rank = match rank_char {
                '2' => CardRank::Two,
                '3' => CardRank::Three,
                '4' => CardRank::Four,
                '5' => CardRank::Five,
                '6' => CardRank::Six,
                '7' => CardRank::Seven,
                '8' => CardRank::Eight,
                '9' => CardRank::Nine,
                'T' => CardRank::Ten,
                'J' => CardRank::Jack,
                'Q' => CardRank::Queen,
                'K' => CardRank::King,
                'A' => CardRank::Ace,
                x => return Err(CardError::InvalidRank(x)),
            };

            let suit_char = chars.next().unwrap();
            let suit = match suit_char {
                'C' => Suit::Clubs,
                'D' => Suit::Diamonds,
                'H' => Suit::Hearts,
                'S' => Suit::Spades,
                x => return Err(CardError::InvalidSuit(x)),
            };

            Ok(Card {
                rank: rank,
                suit: suit,
                _s: (),
            })
        } else {
            Err(CardError::WrongLength)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note that you can write impl Struct as much as you want, in any file.
    // The implementation will only become accessible / compiled if you import the module.
    // Which means, I can add this impl behind a test build flag.
    // This code is only visible and accessible when running tests, and is compiled out in shipping.
    impl Card {
        pub fn new(rank: CardRank, suit: Suit) -> Card {
            Card {
                rank: rank,
                suit: suit,
                _s: (),
            }
        }
    }
}
