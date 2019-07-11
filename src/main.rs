#![allow(dead_code)]

fn main() {}

struct Game {}

enum RollStatus {
    First,
    Second,
    Third,
    Invalid,
}

impl Game {
    fn new() -> Game {
        Game {}
    }

    fn roll(&mut self, pins: u8) -> RollStatus {
        RollStatus::Invalid
    }
    fn score(&self) -> u16 {
        0
    }
}

#[cfg(test)]
mod tests {
    extern crate rstest;
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn new_game_call_score_score_is_0() {
        let game = Game::new();
        assert_eq!(0, game.score());
    }
}
