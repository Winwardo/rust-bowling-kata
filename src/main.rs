#![allow(dead_code)]

fn main() {}

struct Game {}

enum RollStatus {
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

    #[rstest]
    fn first_roll_is_0_score_is_0() {
        let mut game = Game::new();
        game.roll(0);
        assert_eq!(0, game.score());
    }

    #[rstest]
    fn first_roll_is_9_score_is_9() {
        let mut game = Game::new();
        game.roll(9);
        assert_eq!(9, game.score());
    }

}
