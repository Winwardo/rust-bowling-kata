fn main() {}

struct Game {}

impl Game {
    fn new() -> Game {
        Game {}
    }

    fn roll(pins: u8) {}
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
    fn new_game__call_score__score_is_0() {
        let game = Game::new();
        assert_eq!(0, game.score());
    }
}
