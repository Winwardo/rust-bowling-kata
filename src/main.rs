#[allow(dead_code)]
fn bowl() -> bool {
    false
}

fn main() {
    
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope
    extern crate rstest;
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn base() {
        assert_eq!(false, bowl());
    }
}
