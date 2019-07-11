fn main() {}

type GameScore = u64;

struct Game {
    rolls: Vec<GameScore>,
}

struct Score {
    score: GameScore,
    advance_by: usize,
}

fn score_frame(roll_1: GameScore, roll_2: GameScore, roll_3: GameScore) -> Score {
    if roll_1 == 10 {
        // frame is a strike
        Score {
            score: roll_1 + roll_2 + roll_3,
            advance_by: 1,
        }
    } else if roll_1 + roll_2 == 10 {
        // frame is a spare
        Score {
            score: roll_1 + roll_2 + roll_3,
            advance_by: 2,
        }
    } else {
        Score {
            score: roll_1 + roll_2,
            advance_by: 2,
        }
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

        dbg!(rolls.len());

        let mut idx = 0;
        let mut score = 0;

        loop {
            let last_idx = idx;

            if idx >= rolls.len() {
                break;
            }

            let rolls_left = rolls.len() - idx;

            let out = match rolls_left {
                1 => score_frame(rolls[idx], 0, 0),
                2 => score_frame(rolls[idx], rolls[idx + 1], 0),
                3 => score_frame(rolls[idx], rolls[idx + 1], rolls[idx + 2]),
                _ => score_frame(rolls[idx], rolls[idx + 1], rolls[idx + 2]),
            };

            score += out.score;
            idx += out.advance_by;

            if idx == last_idx {
                panic!("Did not advance");
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

    // #[rstest]
    // fn

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
