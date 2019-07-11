use super::card::*;
use super::comparison_result::*;
use super::hand::*;

#[derive(Debug, PartialEq)]
pub struct CompareHands {
    pub black: Hand,
    pub white: Hand,
}

fn find_pair(cards: &Vec<Card>) -> Option<(&Card, &Card)> {
    cards.windows(2).fold(None, |acc, slice| {
        if acc.is_none() {
            let left = &slice[0];
            let right = &slice[1];

            if left.rank == right.rank {
                return Some((&left, &right));
            }
        }

        acc
    })
}

fn high_card(sorted_black: &Vec<Card>, sorted_white: &Vec<Card>) -> Option<Winner> {
    let highest_rank_black = &sorted_black
        .iter()
        .max()
        .expect("Black cards should have at least one card.");
    let highest_rank_white = &sorted_white
        .iter()
        .max()
        .expect("White cards should have at least one card.");

    if highest_rank_black > highest_rank_white {
        return Some(Winner {
            player: Players::Black,
            win_type: WinType::HighCard(highest_rank_black.rank),
        });
    } else if highest_rank_black < highest_rank_white {
        return Some(Winner {
            player: Players::White,
            win_type: WinType::HighCard(highest_rank_white.rank),
        });
    } else {
        None
    }
}

fn pair(sorted_black: &Vec<Card>, sorted_white: &Vec<Card>) -> Option<Winner> {
    let black_pair_opt = find_pair(&sorted_black);
    let white_pair_opt = find_pair(&sorted_white);

    if let Some(black_pair) = black_pair_opt {
        // Black has a pair
        let (black1, _black2) = black_pair;

        // Note that I inlined the destructuring for white here - I think it's harder to read though
        if let Some((white1, _white2)) = white_pair_opt {
            // White has a pair
            if black1 > white1 {
                // Black wins with a better pair
                Some(Winner {
                    player: Players::Black,
                    win_type: WinType::Pair(black1.rank),
                })
            } else {
                // Both have equal pairs
                // Let's search through the lower cards for a high
                // First remove the existing pair cards, then just reuse the HighCard algorithm
                let target_rank = black1.rank;

                // No reason this closure couldn't have been a separate free-standing function, I just wanted it here
                let remove_pair = |v: &Vec<Card>| {
                    v.into_iter()
                        .filter(|card| card.rank != target_rank)
                        .cloned() // This explicitly clones each card, so we can return Vec<Card> - otherwise we'd have to return Vec<&Card>, as we haven't marked Card as Copy
                        .collect()
                };

                let filtered_black = remove_pair(&sorted_black);
                let filtered_white = remove_pair(&sorted_white);

                // Let's be clear: This is some stupid ass code about to happen.
                // if high_card is None, then all this code is ignored and we return None
                high_card(&filtered_black, &filtered_white).and_then(|winner| {
                    // Same here - if result.winner is None, ignore the .and_then and propagate the None

                    // Match the winner.win_type against HighCard - if it's any other value, you guessed it, propagate None.
                    match winner.win_type {
                        // We destructure to pull the rank value out, and reuse it to convert from HighCard(rank) to PairHighCard(rank)
                        WinType::HighCard(rank) => Some(Winner {
                            player: winner.player,
                            win_type: WinType::PairHighCard(rank),
                        }),
                        _ => None,
                    }
                })
            }
        } else {
            // Black has a pair, white does not
            Some(Winner {
                player: Players::Black,
                win_type: WinType::Pair(black1.rank),
            })
        }
    } else {
        // Black does not have a pair
        if let Some((white1, _white2)) = white_pair_opt {
            Some(Winner {
                player: Players::White,
                win_type: WinType::Pair(white1.rank),
            })
        } else {
            None
        }
    }
}

impl CompareHands {
    pub fn compare(&self) -> ComparisonResult {
        let sorted_black = sort_and_reverse(&self.black.cards);
        let sorted_white = sort_and_reverse(&self.white.cards);

        // Check each possible win type
        None.or(pair(&sorted_black, &sorted_white))
            .or(high_card(&sorted_black, &sorted_white))
            .map(|winner| ComparisonResult::from_winner(winner)) // Convert to a comparison result if we got a winner
            .unwrap_or(ComparisonResult::tie()) // Otherwise it must be a tie
    }
}

fn sort_and_reverse<T>(vec: &Vec<T>) -> Vec<T>
where
    T: std::cmp::Ord + std::clone::Clone,
{
    let mut result = vec.clone();
    result.sort();
    result.reverse();
    result.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::card_rank::*;
    use super::super::suits::*;

    #[test]
    fn same_hands_no_winner() {
        let input = CompareHands {
            black: Hand::new(vec![
                Card::new(CardRank::Two, Suit::Hearts),
                Card::new(CardRank::Three, Suit::Hearts),
                Card::new(CardRank::Four, Suit::Hearts),
                Card::new(CardRank::Five, Suit::Hearts),
                Card::new(CardRank::Six, Suit::Hearts),
            ])
            .unwrap(),
            white: Hand::new(vec![
                Card::new(CardRank::Two, Suit::Clubs),
                Card::new(CardRank::Three, Suit::Clubs),
                Card::new(CardRank::Four, Suit::Clubs),
                Card::new(CardRank::Five, Suit::Clubs),
                Card::new(CardRank::Six, Suit::Clubs),
            ])
            .unwrap(),
        }
        .compare();

        assert_eq!(input, ComparisonResult::tie());
    }

    #[test]
    fn high_card_black() {
        let input = CompareHands {
            black: Hand::new(vec![
                Card::new(CardRank::Two, Suit::Hearts),
                Card::new(CardRank::Three, Suit::Hearts),
                Card::new(CardRank::Four, Suit::Hearts),
                Card::new(CardRank::Five, Suit::Hearts),
                Card::new(CardRank::Ace, Suit::Hearts),
            ])
            .unwrap(),
            white: Hand::new(vec![
                Card::new(CardRank::Two, Suit::Clubs),
                Card::new(CardRank::Three, Suit::Clubs),
                Card::new(CardRank::Four, Suit::Clubs),
                Card::new(CardRank::Five, Suit::Clubs),
                Card::new(CardRank::Six, Suit::Clubs),
            ])
            .unwrap(),
        }
        .compare();

        assert_eq!(
            input,
            ComparisonResult::make(Players::Black, WinType::HighCard(CardRank::Ace))
        );
    }

    #[test]
    fn pair_black_beats_white_high_card() {
        let input = CompareHands {
            black: Hand::new(vec![
                Card::new(CardRank::Two, Suit::Hearts),
                Card::new(CardRank::Two, Suit::Diamonds),
                Card::new(CardRank::Four, Suit::Hearts),
                Card::new(CardRank::Five, Suit::Hearts),
                Card::new(CardRank::Six, Suit::Hearts),
            ])
            .unwrap(),
            white: Hand::new(vec![
                Card::new(CardRank::Two, Suit::Clubs),
                Card::new(CardRank::Three, Suit::Spades),
                Card::new(CardRank::Four, Suit::Clubs),
                Card::new(CardRank::Five, Suit::Clubs),
                Card::new(CardRank::Seven, Suit::Clubs),
            ])
            .unwrap(),
        }
        .compare();

        assert_eq!(
            input,
            ComparisonResult::make(Players::Black, WinType::Pair(CardRank::Two))
        );
    }

    #[test]
    fn pair_both_white_high_card_that_isnt_in_pair_wins() {
        let input = CompareHands {
            black: Hand::new(vec![
                Card::new(CardRank::Ace, Suit::Hearts),
                Card::new(CardRank::Ace, Suit::Diamonds),
                Card::new(CardRank::Four, Suit::Hearts),
                Card::new(CardRank::Five, Suit::Hearts),
                Card::new(CardRank::Six, Suit::Hearts),
            ])
            .unwrap(),
            white: Hand::new(vec![
                Card::new(CardRank::Ace, Suit::Clubs),
                Card::new(CardRank::Ace, Suit::Spades),
                Card::new(CardRank::Four, Suit::Clubs),
                Card::new(CardRank::Five, Suit::Clubs),
                Card::new(CardRank::Seven, Suit::Clubs),
            ])
            .unwrap(),
        }
        .compare();

        assert_eq!(
            input,
            ComparisonResult::make(Players::White, WinType::PairHighCard(CardRank::Seven))
        );
    }
}
