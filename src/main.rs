type GameScore = u64;

#[derive(Debug)]
enum FrameType {
    Regular(GameScore, GameScore),
    Spare(GameScore, GameScore),
    Strike,
}

#[derive(Debug)]
struct Frame {
    roll_1: GameScore,
    roll_2: Option<GameScore>,
    roll_3: Option<GameScore>,
}

impl Frame {
    pub fn is_strike(&self) -> bool {
        self.roll_1 == STRIKE_SCORE
    }
    pub fn is_spare(&self) -> bool {
        match self.roll_2 {
            Some(x) => self.roll_1 + x == STRIKE_SCORE,
            None => false,
        }
    }
}

const MAX_FRAMES: usize = 10;
const MAX_ROLL_COUNT: usize = (MAX_FRAMES * 2) + 1;

const STRIKE_SCORE: GameScore = 10;

struct Game {
    // current_frame: Option<FrameType>,
    //last_roll: Option<GameScore>,
    past_frames: Vec<Frame>,
}

#[allow(dead_code)]
impl Game {
    fn new() -> Game {
        Game {
            past_frames: Vec::new(),
        }
    }

    fn roll(&mut self, pins: GameScore) {
        dbg!(pins);
        match self.past_frames.last_mut() {
            Some(ref mut frame) => {
                println!("Some frame");
                if frame.roll_1 == STRIKE_SCORE {
                    println!("strike");
                    let frame = Frame {
                        roll_1: pins,
                        roll_2: None,
                        roll_3: None,
                    };
                    self.past_frames.push(frame);
                } else {
                    if frame.roll_2.is_some() {
                        let frame = Frame {
                            roll_1: pins,
                            roll_2: None,
                            roll_3: None,
                        };
                        self.past_frames.push(frame);
                    } else {
                        println!("not a strike");
                        frame.roll_2 = Some(pins);
                    }
                }
            }
            _ => {
                println!("empty frame");
                let frame = Frame {
                    roll_1: pins,
                    roll_2: None,
                    roll_3: None,
                };
                self.past_frames.push(frame);
            }
        };
    }

    fn score(&self) -> GameScore {
        println!("======== score");
        // match self.past_frames.last() {
        //     Some(frame) => frame.roll_1 + frame.roll_2.unwrap_or(0) + frame.roll_3.unwrap_or(0),
        //     _ => 0,
        // }

        // dbg!(self.last_roll);
        let mut score = 0;

        dbg!(&self.past_frames);

        for frame_id in 0..self.past_frames.len() {
            dbg!(score);
            println!("checking frame {}", frame_id);
            //     dbg!(frame_id);
            let current = &self.past_frames[frame_id];
            let next = self.past_frames.get(frame_id + 1);
            let next_2 = self.past_frames.get(frame_id + 2);

            dbg!(current);

            if current.is_strike() {
                println!("current strike");
                score += STRIKE_SCORE;
                if let Some(next) = next {
                    score += next.roll_1;
                    if next.is_strike() {
                        if let Some(next_2) = next_2 {
                            score += next_2.roll_1;
                        } else {
                            score += next.roll_2.unwrap_or(0);
                        }
                    } else if next.is_spare() {
                        score += next.roll_2.unwrap_or(0);
                        // score += next.
                    }
                }

            // + match next {
            //     Some(frame) => frame.roll_1,
            //     None => 0,
            // }
            } else if current.is_spare() {
                println!("current spare");
                dbg!(next);
                score += STRIKE_SCORE;

                if let Some(next) = next {
                    score += next.roll_1;
                }
            } else {
                println!("current regular");
                score += current.roll_1 + current.roll_2.unwrap_or(0);
            }

            //     dbg!(current);

            //     // score += match current {
            //     //     FrameType::Regular(first, second) => *first + *second,
            //     //     FrameType::Spare(_, _) => match next {
            //     //         Some(FrameType::Regular(first, _)) => STRIKE_SCORE + first,
            //     //         Some(FrameType::Spare(first, _)) => STRIKE_SCORE + first,
            //     //         Some(FrameType::Strike) => STRIKE_SCORE + STRIKE_SCORE,
            //     //         None => STRIKE_SCORE + self.last_roll.unwrap_or(0),
            //     //     },
            //     //     FrameType::Strike => match next {
            //     //         Some(FrameType::Regular(first, second)) => STRIKE_SCORE + first + second,
            //     //         Some(FrameType::Spare(first, second)) => STRIKE_SCORE + first + second,
            //     //         Some(FrameType::Strike) => match next_2 {
            //     //             Some(FrameType::Regular(first, _)) => STRIKE_SCORE + STRIKE_SCORE + first,
            //     //             Some(FrameType::Spare(first, _)) => STRIKE_SCORE + STRIKE_SCORE + first,
            //     //             Some(FrameType::Strike) => STRIKE_SCORE + STRIKE_SCORE + STRIKE_SCORE,
            //     //             None => STRIKE_SCORE + STRIKE_SCORE + self.last_roll.unwrap_or(0),
            //     //         },
            //     //         None => STRIKE_SCORE + self.last_roll.unwrap_or(0),
            //     //     },
            //     // };

            //     // let (current, next) = frames[0]
            //     // if frame_id > self.frames.
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
