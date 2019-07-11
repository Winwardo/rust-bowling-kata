use failure::{Error, Fail};
use regex::Regex;

use super::card::*;
use super::compare_hands::*;
use super::hand::*;

#[derive(Debug, Fail, PartialEq)]
pub enum DeserializeError {
    #[fail(display = "The given string could not be parsed by the regex.")]
    BadFormat,
    #[fail(display = "The given hand could not be deserialized.")]
    InvalidHand(HandError),
    #[fail(display = "The given cards were not in a valid.")]
    InvalidCards(CardError),
}

fn text_to_cards(text: &str) -> Result<Vec<Card>, CardError> {
    let cards = text
        .split(" ")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|card_str| Card::from_string(card_str));

    let mut result: Vec<Card> = Vec::new();
    for card in cards {
        if card.is_err() {
            return Err(card.unwrap_err());
        } else {
            result.push(card.unwrap());
        }
    }

    Ok(result)
}

fn deserialize_cards_to_hand(text: &str) -> Result<Hand, DeserializeError> {
    Hand::new(text_to_cards(text).map_err(|e| DeserializeError::InvalidCards(e))?)
        .map_err(|e| DeserializeError::InvalidHand(e))
}

pub fn deserialize(input: &str) -> Result<CompareHands, Error> {
    let re = Regex::new(r"^Black: (.*)  White: (.*)$").expect("Invalid Regex in deserializer.");

    re.captures(input)
        .and_then(|captures| {
            // Did we match something after "Black" and after "White"?
            let black = captures.get(1)?;
            let white = captures.get(2)?;

            Some((black.as_str(), white.as_str()))
        })
        .ok_or(Err(DeserializeError::BadFormat)?)
        .and_then(|matches| {
            // Convert text to lists of Card
            let (black_text, white_text) = matches;

            let black_hand = deserialize_cards_to_hand(black_text)?;
            let white_hand = deserialize_cards_to_hand(white_text)?;

            Ok(CompareHands {
                black: black_hand,
                white: white_hand,
            })
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::card_rank::*;
    use super::super::suits::*;

    #[test]
    fn simple_hand_deserializes() {
        let input = "Black: 2H 3D 5S 9C KD  White: 2C 3H 4S 8C AH";
        let expected = CompareHands {
            black: Hand::new(vec![
                Card::new(CardRank::Two, Suit::Hearts),
                Card::new(CardRank::Three, Suit::Diamonds),
                Card::new(CardRank::Five, Suit::Spades),
                Card::new(CardRank::Nine, Suit::Clubs),
                Card::new(CardRank::King, Suit::Diamonds),
            ])
            .unwrap(),
            white: Hand::new(vec![
                Card::new(CardRank::Two, Suit::Clubs),
                Card::new(CardRank::Three, Suit::Hearts),
                Card::new(CardRank::Four, Suit::Spades),
                Card::new(CardRank::Eight, Suit::Clubs),
                Card::new(CardRank::Ace, Suit::Hearts),
            ])
            .unwrap(),
        };

        assert_eq!(Ok(expected), deserialize(&input));
    }

    #[test]
    fn bad_format_errors() {
        let input = "Bad format";
        assert_eq!(Err(DeserializeError::BadFormat), deserialize(&input));
    }

    #[test]
    fn bad_hand_black_has_too_few() {
        let input = "Black: 2H 3D 5S 9C  White: 2C 3H 4S 8C AH";
        assert_eq!(
            Err(DeserializeError::InvalidHand(HandError::NotEnoughCards)),
            deserialize(&input)
        );
    }

    #[test]
    fn bad_card_invalid_suit() {
        let input = "Black: 2Z 3D 5S 9C KD  White: 2C 3H 4S 8C AH";
        assert_eq!(
            Err(DeserializeError::InvalidCards(CardError::InvalidSuit('Z'))),
            deserialize(&input)
        );
    }
}
