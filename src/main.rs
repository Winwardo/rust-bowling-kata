#![allow(dead_code)]

fn main() {}

type GameScore = u64;

struct Game {
    current_score: GameScore,
}

enum RollStatus {
    Invalid,
}

impl Game {
    fn new() -> Game {
        Game { current_score: 0 }
    }

    fn roll(&mut self, pins: u8) -> RollStatus {
        self.current_score += pins as GameScore;

        RollStatus::Invalid
    }

    fn score(&self) -> GameScore {
        self.current_score
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

    #[rstest]
    fn first_roll_is_10_score_is_10() {
        let mut game = Game::new();
        game.roll(10);
        assert_eq!(10, game.score());
    }

    #[rstest]
    fn twenty_0_in_a_row_score_is_0() {
        let mut game = Game::new();
        (0..20).for_each(|_| {
            game.roll(0);
        });
        assert_eq!(0, game.score());
    }

    #[rstest]
    fn spare_then_zero_score_is_10() {
        let mut game = Game::new();

        game.roll(5);
        game.roll(5);
        game.roll(0);

        assert_eq!(10, game.score());
    }

    #[rstest]
    fn strike_then_zero_score_is_10() {
        let mut game = Game::new();

        game.roll(10);
        game.roll(0);

        assert_eq!(10, game.score());
    }
}
