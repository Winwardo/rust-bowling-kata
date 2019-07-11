use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Ord, Clone, Copy)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl PartialOrd for Suit {
    fn partial_cmp(&self, _: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}
