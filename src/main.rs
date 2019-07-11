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
    rolls: Vec<GameScore>,
    current_score: GameScore,
    last_frame: Option<Frame>,
    current_frame: Frame,
}

struct Score {
    score: GameScore,
    advance_by: usize,
}

fn score_4(roll_1: GameScore, roll_2: GameScore, roll_3: GameScore, roll_4: GameScore) -> Score {
    let mut score = roll_1 + roll_2 + roll_3 + roll_4;
    let mut advance_by = 2;

    if roll_1 == 10 {
        // it's a strike
        score += roll_2 + roll_3;
        advance_by = 1;
    } else {
        if roll_1 + roll_2 == 10 {
            // it's a spare
            score += roll_3;
            advance_by = 2;
        }
    }

    Score {
        score: score,
        advance_by: advance_by,
    }
}

impl Game {
    fn new() -> Game {
        Game {
            rolls: Vec::new(),
            current_score: 0,
            last_frame: None,
            current_frame: Frame::NoBowls,
        }
    }

    fn roll(&mut self, pins: GameScore) -> RollStatus {
        self.rolls.push(pins);
        RollStatus::Invalid
    }

    fn score(&self) -> GameScore {
        //let lastState = Frame::NoBowls;

        // for x

        // self.rolls.iter().windows(2);
        // let mut frames = Vec::<GameScore>new();

        let rolls = &self.rolls;

        // if rolls.len() == 0 {
        //     return 0;
        // } else {

        // }

        dbg!(rolls.len());

        let mut idx = 0;
        let mut score = 0;

        loop {
            let last_idx = idx;

            let rolls_left = rolls.len() - idx;
            dbg!(rolls_left);

            let out = match rolls_left {
                0 => score_4(0, 0, 0, 0),
                1 => score_4(rolls[idx], 0, 0, 0),
                2 => score_4(rolls[idx], rolls[idx + 1], 0, 0),
                3 => score_4(rolls[idx], rolls[idx + 1], rolls[idx + 2], 0),
                _ => score_4(rolls[idx], rolls[idx + 1], rolls[idx + 2], rolls[idx + 3]),
                //_ => panic!("No"),
            };

            score += out.score;
            idx += out.advance_by;

            if idx == last_idx {
                panic!("Did not advance");
            }

            break;
        }

        score

        // match rolls.len() {
        //     // 0 => 0,
        //     // 1 => rolls[0],
        //     // 2 => score_2(rolls[0], rolls[1]),
        //     // 3 => score_3(rolls[0], rolls[1], rolls[2]),
        //     _ => {

        //         // let mut idx = 0;
        //         // let mut total = 0;

        //         // loop {
        //         //     let last_idx = idx;
        //         //     let mut score = rolls[idx + 0] + rolls[idx + 1] + rolls[idx + 2];

        //         //     if rolls[idx + 0] == 10 {
        //         //         // it's a strike
        //         //         score += rolls[idx + 1] + rolls[idx + 2];
        //         //         idx += 1;
        //         //     } else {
        //         //         if rolls[idx + 0] + rolls[idx + 1] == 10 {
        //         //             // it's a spare
        //         //             score += rolls[idx + 2];
        //         //             idx += 2;
        //         //         }
        //         //     }

        //         //     total += score;
        //         // }

        //         // total
        //     }
        // }

        // let mut score = rolls[0] + rolls[1];

        // let mut count = 2;
        // let mut counted_spare = false;

        // loop {
        //     // work in chunks

        //     let last_count = count;
        //     if count >= rolls.len() {
        //         break;
        //     }
        //     println!("------ {}", count);

        //     // let roll_1 = rolls[count];
        //     // let roll_2 = rolls[count + 1];
        //     // let roll_3 = rolls[count + 2];
        //     // let roll_4 = rolls[count + 3];

        //     let was_spare = (rolls[count] + rolls[count + 1]) == 10;
        //     if was_spare {}

        //     // // if count == 0 {
        //     // //     score += rolls[0];
        //     // //     count += 1;
        //     // // } else if count == 1 {
        //     // //     score += rolls[1];
        //     // //     count += 1;
        //     // // } else
        //     // {
        //     //     let was_strike = rolls[count - 1] == 10;
        //     //     if was_strike {
        //     //         count += 1;
        //     //     } else {
        //     // let was_spare = (rolls[count - 1] + rolls[count - 2]) == 10;
        //     //         dbg!(was_spare);
        //     //         if was_spare {
        //     //             if counted_spare == false {
        //     //                 score += rolls[count];
        //     //                 score += rolls[count];
        //     //                 count += 1;
        //     //                 counted_spare = true;
        //     //             } else {
        //     //                 counted_spare = false;
        //     //             }
        //     //         } else {
        //     //             score += rolls[count];
        //     //             count += 1;
        //     //         }
        //     //     }

        //     //     //let last_frame_score = rolls[count-1] + rolls[count-2];
        //     //     //if
        //     // }

        //     if count == last_count {
        //         panic!("ahh");
        //     }
        // }

        // score
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

        assert_eq!(50, game.score());
    }
}
