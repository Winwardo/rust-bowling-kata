use std::collections::BTreeSet;
use std::iter::FromIterator;

use super::card::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    pub cards: Vec<Card>,
    _c: (),
}

#[derive(Debug, PartialEq)]
pub enum HandError {
    NotEnoughCards,
    TooManyCards,
    DuplicatedCards,
}

impl Hand {
    pub fn new(cards: Vec<Card>) -> Result<Hand, HandError> {
        match cards.len() {
            x if x < 5 => Err(HandError::NotEnoughCards),
            x if x > 5 => Err(HandError::TooManyCards),
            x => {
                // Check for duplicate cards
                if BTreeSet::from_iter(cards.clone()).len() == x {
                    Ok(Hand {
                        cards: cards,
                        _c: (),
                    })
                } else {
                    Err(HandError::DuplicatedCards)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::card_rank::*;
    use super::super::suits::*;

    #[test]
    fn simple_hand_works() {
        let input = Hand::new(vec![
            Card::new(CardRank::Two, Suit::Hearts),
            Card::new(CardRank::Three, Suit::Hearts),
            Card::new(CardRank::Four, Suit::Hearts),
            Card::new(CardRank::Five, Suit::Hearts),
            Card::new(CardRank::Six, Suit::Hearts),
        ]);

        assert!(input.is_ok());
    }

    #[test]
    fn not_enough_cards() {
        let input = Hand::new(vec![
            Card::new(CardRank::Two, Suit::Hearts),
            Card::new(CardRank::Three, Suit::Hearts),
            Card::new(CardRank::Four, Suit::Hearts),
            Card::new(CardRank::Five, Suit::Hearts),
        ]);

        assert_eq!(Err(HandError::NotEnoughCards), input);
    }

    #[test]
    fn too_many_cards() {
        let input = Hand::new(vec![
            Card::new(CardRank::Two, Suit::Hearts),
            Card::new(CardRank::Three, Suit::Hearts),
            Card::new(CardRank::Four, Suit::Hearts),
            Card::new(CardRank::Five, Suit::Hearts),
            Card::new(CardRank::Six, Suit::Hearts),
            Card::new(CardRank::Seven, Suit::Hearts),
        ]);

        assert_eq!(Err(HandError::TooManyCards), input);
    }

    #[test]
    fn duplicate_cards() {
        let input = Hand::new(vec![
            Card::new(CardRank::Two, Suit::Hearts),
            Card::new(CardRank::Two, Suit::Hearts),
            Card::new(CardRank::Four, Suit::Hearts),
            Card::new(CardRank::Five, Suit::Hearts),
            Card::new(CardRank::Six, Suit::Hearts),
        ]);

        assert_eq!(Err(HandError::DuplicatedCards), input);
    }
}
