fn main() {}

type GameScore = u64;

struct Game {
    rolls: Vec<GameScore>,
}

enum FrameType {
    Regular,
    Spare,
    Strike,
}

fn score_frame(roll_1: GameScore, roll_2: GameScore, roll_3: GameScore) -> (FrameType, GameScore) {
    if roll_1 == 10 {
        (FrameType::Strike, roll_1 + roll_2 + roll_3)
    } else if roll_1 + roll_2 == 10 {
        (FrameType::Spare, roll_1 + roll_2 + roll_3)
    } else {
        (FrameType::Regular, roll_1 + roll_2)
    }
}

#[allow(dead_code)]
impl Game {
    fn new() -> Game {
        Game { rolls: Vec::new() }
    }

    fn roll(&mut self, pins: GameScore) {
        self.rolls.push(pins);
    }

    fn score(&self) -> GameScore {
        let rolls = &self.rolls;

        let mut roll_id = 0;
        let mut frame_id = 0;
        let mut score = 0;

        loop {
            let last_roll_id = roll_id;

            if roll_id >= rolls.len() {
                break;
            }
            let rolls_left = rolls.len() - roll_id;

            if frame_id == 10 {
                println!("10th frame!!");
            }

            let (frame_type, frame_score) = match rolls_left {
                1 => score_frame(rolls[roll_id], 0, 0),
                2 => score_frame(rolls[roll_id], rolls[roll_id + 1], 0),
                3 => score_frame(rolls[roll_id], rolls[roll_id + 1], rolls[roll_id + 2]),
                _ => score_frame(rolls[roll_id], rolls[roll_id + 1], rolls[roll_id + 2]),
            };

            score += frame_score;
            roll_id += match frame_type {
                FrameType::Regular | FrameType::Spare => 2,
                FrameType::Strike => 1,
            };
            frame_id += 1;

            assert_ne!(last_roll_id, roll_id, "Did not advance.");
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
    fn twenty_0_in_a_row_score_is_0() {
        let mut game = Game::new();

        for _ in 0..20 {
            game.roll(0);
        }

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
    fn twelve_strikes_scores_300q() {
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
    fn ten_pairs_of_5_and_spare_then_final_5_scores_150q() {
        let mut game = Game::new();

        for _ in 0..10 {
            game.roll(5);
            game.roll(5);
        }
        game.roll(5);

        assert_eq!(150, game.score());
    }

    // #[rstest]
    // fn full_example_game() {
    //     let mut game = Game::new();

    //     game.roll(1);
    //     game.roll(4);
    //     game.roll(4);
    //     game.roll(5);
    //     game.roll(6);
    //     game.roll(4);
    //     game.roll(5);
    //     game.roll(5);
    //     game.roll(10);
    //     game.roll(0);
    //     game.roll(1);
    //     game.roll(7);
    //     game.roll(3);
    //     game.roll(6);
    //     game.roll(4);
    //     game.roll(10);
    //     game.roll(2);
    //     game.roll(8);
    //     game.roll(6);

    //     assert_eq!(133, game.score());
    // }
}
