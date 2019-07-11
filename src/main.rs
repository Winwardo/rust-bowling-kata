#![allow(dead_code)]

fn main() {}

type GameScore = u64;

enum RollStatus {
    Invalid,
}

enum FrameStatus {
    None,
    FirstBowl(GameScore),
    SecondBowl(GameScore),
    Spare,
    Strike,
}

struct Game {
    current_score: GameScore,
    last_frame: FrameStatus,
}

impl Game {
    fn new() -> Game {
        Game {
            current_score: 0,
            last_frame: FrameStatus::None,
        }
    }

    fn roll(&mut self, pins: GameScore) -> RollStatus {
        self.current_score += pins;

        match &self.last_frame {
            FrameStatus::None => {
                self.last_frame = FrameStatus::FirstBowl(pins);
            }
            FrameStatus::FirstBowl(last_score) => {
                if last_score + pins == 10 {
                    self.last_frame = FrameStatus::Spare;
                } else {
                    self.last_frame = FrameStatus::SecondBowl(pins);
                }
            }
            FrameStatus::Spare => {
                self.current_score += pins;
            }
            _ => {}
        }

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

        game.roll(4);
        game.roll(6);
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

    #[rstest]
    fn spare_adds_next_score() {
        let mut game = Game::new();

        game.roll(4);
        game.roll(6);
        game.roll(5);

        assert_eq!(20, game.score());
    }
}
