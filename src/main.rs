#![allow(dead_code)]

fn main() {}

type GameScore = u64;

enum RollStatus {
    Invalid,
}

#[derive(Clone, Copy, Debug)]
enum Frame {
    NoBowls,
    OneBowl(GameScore),
    TwoBowls(GameScore, GameScore),
    Spare(GameScore, GameScore),
    Strike,
}

struct Game {
    current_score: GameScore,
    last_frame: Option<Frame>,
    current_frame: Frame,
}

impl Game {
    fn new() -> Game {
        Game {
            current_score: 0,
            last_frame: None,
            current_frame: Frame::NoBowls,
        }
    }

    fn roll(&mut self, pins: GameScore) -> RollStatus {
        dbg!(("Roll", pins));

        let bonus = match dbg!(&self.last_frame) {
            Some(last_frame) => match last_frame {
                Frame::Spare(_, _) => match &self.current_frame {
                    Frame::NoBowls => pins,
                    _ => 0,
                },
                Frame::Strike => match &self.current_frame {
                    Frame::NoBowls | Frame::OneBowl(_) => pins,
                    _ => 0,
                },
                _ => 0,
            },
            None => 0,
        };

        self.current_frame = match &self.current_frame {
            Frame::NoBowls => Frame::OneBowl(pins),
            Frame::OneBowl(first_pins) => {
                if *first_pins + pins == 10 {
                    dbg!(("Spare!", *first_pins, pins));
                    Frame::Spare(*first_pins, pins)
                } else {
                    Frame::TwoBowls(*first_pins, pins)
                }
            }
            _ => {
                panic!(
                    "The current frame cannot have more than one bowl before moving to the next."
                );
            }
        };

        match &self.current_frame {
            Frame::TwoBowls(_, _) | Frame::Spare(_, _) | Frame::Strike => {
                dbg!("Swap over");
                self.last_frame = Some(self.current_frame.clone());
                self.current_frame = Frame::NoBowls;
            }
            _ => {}
        }

        dbg!(bonus);

        self.current_score += pins + bonus;

        // match &self.last_frame {
        //     Some(last_frame) => {
        //         match &self.last_frame {
        //             Frame::OneBowl, Frame::TwoBowls => {

        //             }
        //         }
        //     }
        //     None => {
        //         // First frame

        //     }
        // }

        // if self.frames.is_empty() {
        //     self.frames.push(Frame::new())
        // }

        // let mut current_frame = self.frames.last_mut().unwrap();
        // match current_frame.roll_1 {
        //     Some(frame) => {

        //     }
        //     None => {
        //         current_frame.roll_1 = Some(pins);
        //         //panic!("whoops");
        //     }
        // }

        // match &self.last_frame {
        //     FrameStatus::None => {
        //         self.last_frame = FrameStatus::FirstBowl(pins);
        //     }
        //     FrameStatus::FirstBowl(last_frame_score) => {
        //         if last_frame_score + pins == 10 {
        //             self.last_frame = FrameStatus::Spare;
        //         } else {
        //             self.last_frame = FrameStatus::SecondBowl(pins);
        //         }
        //     }
        //     FrameStatus::SecondBowl(_) => {
        //         self.last_frame = FrameStatus::None;
        //     }
        //     FrameStatus::Spare => {
        //         self.current_score += pins;
        //     }
        //     _ => {}
        // }

        RollStatus::Invalid
    }

    fn score(&self) -> GameScore {
        //self.frames.into_iter().map(|x| x.score())
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

        game.roll(4);
        game.roll(6);
        game.roll(4);
        game.roll(6);

        assert_eq!(30, game.score());
    }
}
