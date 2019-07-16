type GameScore = u64;

#[derive(Debug)]
enum FrameType {
    Regular(GameScore, GameScore),
    Spare(GameScore, GameScore),
    Strike,
}

#[derive(Debug, Clone)]
struct Frame {
    roll_1: Option<GameScore>,
    roll_2: Option<GameScore>,
    roll_3: Option<GameScore>,
}

impl Frame {
    pub fn new() -> Frame {
        Frame {
            roll_1: None,
            roll_2: None,
            roll_3: None,
        }
    }
}

const MAX_FRAMES: usize = 10;
const MAX_ROLL_COUNT: usize = (MAX_FRAMES * 2) + 1;

const STRIKE_SCORE: GameScore = 10;

struct Game {
    // current_frame: Option<Frame>,
    //last_roll: Option<GameScore>,
    past_frames: Vec<Frame>,
}

#[allow(dead_code)]
impl Game {
    fn new() -> Game {
        Game {
            past_frames: vec![Frame::new()],
            // current_frame: None,
        }
    }

    fn roll(&mut self, pins: GameScore) {
        // if self.current_frame.is_none() {
        let mut current_frame = self.past_frames.last_mut().unwrap();

        if current_frame.roll_1.is_none() {
            current_frame.roll_1 = Some(pins);

            if pins == STRIKE_SCORE {
                self.past_frames.push(Frame::new());
            };
        } else if current_frame.roll_2.is_none() {
            current_frame.roll_2 = Some(pins);
            self.past_frames.push(Frame::new());
        }

        // match current_frame {
        //     Some(ref mut frame) => {
        //         if frame.roll_1 == STRIKE_SCORE {
        //             // self.past_frames.push(frame.clone());
        //             // self.current_frame = None;
        //         } else {
        //             match frame.roll_2 {
        //                 Some(_) => {
        //                     panic!("Last frame too early");
        //                 }
        //                 None => {
        //                     frame.roll_2 = Some(pins);
        //                     // self.past_frames.push(frame.clone());
        //                     // self.current_frame = None;
        //                 }
        //             };
        //         }
        //     }
        //     // None => {
        //     //     self.past_frames.push(Frame {
        //     //         roll_1: Some(pins),
        //     //         roll_2: None,
        //     //         roll_3: None,
        //     //     });
        //     // }
        // };
    }

    fn score(&self) -> GameScore {
        println!("======== score");

        let mut score = 0;

        dbg!(&self.past_frames);

        for frame_id in 0..self.past_frames.len() {
            dbg!(score);
            let current = &self.past_frames[frame_id];
            let next = self.past_frames.get(frame_id + 1);
            let next_2 = self.past_frames.get(frame_id + 2);

            if current.roll_1.unwrap_or(0) == STRIKE_SCORE {
                score += STRIKE_SCORE;

                match next {
                    Some(frame_1) => {
                        let a = frame_1.roll_1.unwrap_or(0);
                        score += a;
                        if a == STRIKE_SCORE {
                            match next_2 {
                                Some(frame_2) => {
                                    score += frame_2.roll_1.unwrap_or(0);
                                }
                                None => {}
                            }
                        } else {
                            score += frame_1.roll_2.unwrap_or(0);
                        }
                    }
                    None => {}
                }
            } else if current.roll_1.unwrap_or(0) + current.roll_2.unwrap_or(0) == STRIKE_SCORE {
                score += STRIKE_SCORE;
                score += match next {
                    Some(frame) => frame.roll_1.unwrap_or(0),
                    None => 0,
                }
            } else {
                score += current.roll_1.unwrap_or(0) + current.roll_2.unwrap_or(0);
            }
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
