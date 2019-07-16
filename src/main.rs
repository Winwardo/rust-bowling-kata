type GameScore = u64;

#[derive(Debug)]
enum FrameType {
    OneBowl(GameScore),
    TwoBowl(GameScore, GameScore),
    Spare(GameScore),
    Strike,
}

const MAX_FRAMES: usize = 10;
const MAX_ROLL_COUNT: usize = (MAX_FRAMES * 2) + 1;

const STRIKE_SCORE: GameScore = 10;

struct Game {
    frames: Vec<FrameType>,
}

#[allow(dead_code)]
impl Game {
    fn new() -> Game {
        Game { frames: Vec::new() }
    }

    fn roll(&mut self, pins: GameScore) {
        println!("Roll {}", pins);
        match &self.frames.pop() {
            Some(FrameType::OneBowl(first_pins)) => {
                if first_pins + pins == STRIKE_SCORE {
                    println!("Spare");
                    self.frames.push(FrameType::Spare(*first_pins));
                } else {
                    self.frames.push(FrameType::TwoBowl(*first_pins, pins));
                }
            }
            Some(FrameType::TwoBowl(first, second)) => {
                self.frames.push(FrameType::TwoBowl(*first, *second));

                if pins == STRIKE_SCORE {
                    self.frames.push(FrameType::Strike);
                } else {
                    self.frames.push(FrameType::OneBowl(pins));
                }
            }
            _ => {
                if pins == STRIKE_SCORE {
                    self.frames.push(FrameType::Strike);
                } else {
                    self.frames.push(FrameType::OneBowl(pins));
                }
            }
        };
    }

    fn score(&self) -> GameScore {
        let mut score = 0;

        dbg!(&self.frames);

        for frame_id in 0..self.frames.len() {
            dbg!(frame_id);
            let current = &self.frames[frame_id];

            // if &self.frames.get(frame_id + 1);
            // let next = &self.frames[frame_id + 1];

            dbg!(current);

            score += match current {
                FrameType::OneBowl(pins) => *pins,
                FrameType::TwoBowl(first_pins, second_pins) => *first_pins + *second_pins,
                FrameType::Spare(_) => match self.frames.get(frame_id + 1) {
                    Some(FrameType::OneBowl(pins)) => STRIKE_SCORE + pins,
                    Some(FrameType::TwoBowl(first_pins, _)) => STRIKE_SCORE + first_pins,
                    _ => STRIKE_SCORE,
                },
                FrameType::Strike => match self.frames.get(frame_id + 1) {
                    Some(FrameType::OneBowl(pins)) => STRIKE_SCORE + pins,
                    Some(FrameType::TwoBowl(first_pins, second_pins)) => {
                        STRIKE_SCORE + first_pins + second_pins
                    }
                    _ => STRIKE_SCORE,
                },
            };

            // let (current, next) = frames[0]
            // if frame_id > self.frames.
        }

        score
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
    fn two_rolls_are_4_score_is_8() {
        let mut game = Game::new();
        game.roll(4);
        game.roll(4);
        assert_eq!(8, game.score());
    }

    #[rstest]
    fn three_rolls_are_4_score_is_12() {
        let mut game = Game::new();
        game.roll(4);
        game.roll(4);
        game.roll(4);
        assert_eq!(12, game.score());
    }

    #[rstest]
    fn twenty_0_in_a_row_score_is_0() {
        let mut game = Game::new();

        for _ in 0..20 {
            game.roll(0);
        }

        assert_eq!(0, game.score());
    }

    #[rstest]
    fn eighteen_0_in_a_row_score_is_with_10s_0() {
        let mut game = Game::new();

        for _ in 0..18 {
            game.roll(0);
        }
        game.roll(10);
        game.roll(10);
        game.roll(10);

        assert_eq!(30, game.score());
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
        game.roll(4);

        assert_eq!(18, game.score());
    }

    #[rstest]
    fn spare_adds_next_score_not_score_after_that() {
        let mut game = Game::new();

        game.roll(4);
        game.roll(6);
        game.roll(4);
        game.roll(6);

        assert_eq!(24, game.score());
    }

    #[rstest]
    fn strike_adds_next_two_scores() {
        let mut game = Game::new();

        game.roll(10);
        game.roll(4);
        game.roll(6);

        assert_eq!(30, game.score());
    }

    #[rstest]
    fn two_strikes_adds_next_two_scores() {
        let mut game = Game::new();

        game.roll(10);
        game.roll(10);
        game.roll(4);
        game.roll(4);

        // three frames: [x, x, (4,4)]
        //[10 + 10 + 4, 10 + 4 + 4, 4 + 4]

        assert_eq!(50, game.score());
    }

    #[rstest]
    fn twelve_strikes_scores_300() {
        let mut game = Game::new();

        for _ in 0..12 {
            game.roll(10);
        }

        assert_eq!(300, game.score());
    }

    #[rstest]
    fn ten_pairs_of_9_and_miss_scores_90() {
        let mut game = Game::new();

        for _ in 0..10 {
            game.roll(9);
            game.roll(0);
        }

        assert_eq!(90, game.score());
    }

    #[rstest]
    fn ten_pairs_of_5_and_spare_then_final_5_scores_150() {
        let mut game = Game::new();

        for _ in 0..10 {
            game.roll(5);
            game.roll(5);
        }
        game.roll(5);

        assert_eq!(150, game.score());
    }

    #[rstest]
    fn ninth_frame_is_strike_then_last_frame_is_10_10_9_do_not_double_bonus() {
        let mut game = Game::new();

        for _ in 0..16 {
            game.roll(0)
        }

        game.roll(10);
        game.roll(10);
        game.roll(10);
        game.roll(9);

        assert_eq!(59, game.score());
    }

    #[rstest]
    fn full_example_game() {
        let mut game = Game::new();

        game.roll(1);
        game.roll(4);
        game.roll(4);
        game.roll(5);
        game.roll(6);
        game.roll(4);
        game.roll(5);
        game.roll(5);
        game.roll(10);
        game.roll(0);
        game.roll(1);
        game.roll(7);
        game.roll(3);
        game.roll(6);
        game.roll(4);
        game.roll(10);
        game.roll(2);
        game.roll(8);
        game.roll(6);

        assert_eq!(133, game.score());
    }
}
