fn main() {}

struct Game {}

impl Game {
    fn roll(pins: u8) {}
    fn score() -> u16 {
        0
    }
}

#[cfg(test)]
mod tests {
    extern crate rstest;
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn base() {
        //assert_eq!(false, bowl());
    }
}
