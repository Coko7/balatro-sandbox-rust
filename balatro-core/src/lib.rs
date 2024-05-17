mod playing_card;
mod poker_hands;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::playing_card::{CardRank, CardSuit, PlayingCard};

    #[test]
    fn it_works() {
        let card = PlayingCard::new(CardRank::Number(1), CardSuit::Spade);
        println!("{}", card);

        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
