extern crate regex;

#[macro_use]
extern crate failure;

use failure::Error;

mod poker;

#[allow(dead_code)]
fn poker_hand(input: &str) -> Result<String, Error> {
    poker::deserialize(input)
        .map(|comparable| comparable.compare())
        .and_then(|comparison| poker::serialize(&comparison))
}

fn main() {
    let input = "Black: 2Z 3D 5S 9C KD  White: 2C 3H 4S 8C AH";
    let result = poker_hand(input).unwrap_err();

    match result {
        Ok(x) => println!("{:?}", x),
        Error(e) => println!("{}", e),
    }
    // println!("{}", );
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope
    extern crate rstest;
    use rstest::rstest_parametrize;

    use super::*;

    #[rstest_parametrize(
        expected,
        input,
        case(
            "White wins. - with high card: Ace",
            "Black: 2H 3D 5S 9C KD  White: 2C 3H 4S 8C AH"
        ),
        case(
            "Black wins. - with full house: 4 over 2",
            "Black: 2H 4S 4C 2D 4H  White: 2S 8S AS QS 3S"
        ),
        case(
            "Black wins. - with high card: 9",
            "Black: 2H 3D 5S 9C KD  White: 2C 3H 4S 8C KH"
        ),
        case("Tie.", "Black: 2H 3D 5S 9C KD  White: 2D 3H 5C 9S KH")
    )]
    fn poker_hand_test(expected: &str, input: &str) {
        assert_eq!(expected.to_string(), poker_hand(input).unwrap());
    }
}
