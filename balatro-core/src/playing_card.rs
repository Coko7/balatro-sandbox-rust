use std::fmt;

pub enum CardRank {
    Number(u32),
    Jack,
    Queen,
    King,
}

impl CardRank {
    pub fn get_pos(&self) -> u32 {
        match self {
            CardRank::Number(val) => *val,
            CardRank::Jack => 11,
            CardRank::Queen => 12,
            CardRank::King => 13,
        }
    }

    pub fn get_base_chips(&self) -> u32 {
        match self {
            CardRank::Number(val) if *val == 1 => 11,
            CardRank::Number(val) => *val,
            _ => 10,
        }
    }

    pub fn is_face(&self) -> bool {
        match self {
            CardRank::Number(_) => false,
            _ => true,
        }
    }
}

pub enum CardSuit {
    Spade = 0,
    Heart = 1,
    Club = 2,
    Diamond = 3,
}

pub enum CardEnhancement {
    Bonus,
    Mult,
    Wild,
    Glass,
    Steel,
    Stone,
    Gold,
    Lucky,
}

pub enum CardEdition {
    Base,
    Foil,
    Holographic,
    Polychrome,
    Negative,
}

pub enum CardSeal {
    Gold,
    Red,
    Blue,
    Purple,
}

pub struct PlayingCard {
    rank: CardRank,
    suit: CardSuit,

    edition: CardEdition,
    enhancement: Option<CardEnhancement>,
    seal: Option<CardSeal>,
}

impl PlayingCard {
    pub fn new(rank: CardRank, suit: CardSuit) -> PlayingCard {
        PlayingCard {
            rank,
            suit,
            edition: CardEdition::Base,
            enhancement: None,
            seal: None,
        }
    }

    pub fn is_face_card(&self) -> bool {
        self.rank.is_face()
    }

    pub fn is_even_card(&self) -> bool {
        !self.is_face_card() && self.rank.get_pos() % 2 == 0
    }

    pub fn is_odd_card(&self) -> bool {
        !&self.is_face_card() && self.rank.get_pos() % 2 != 0
    }

    pub fn get_base_chips_value(&self) -> u32 {
        self.rank.get_base_chips()
    }

    pub fn display_value(&self) -> String {
        match self.rank {
            CardRank::Number(val) if val == 1 => "Ace".to_string(),
            CardRank::Number(val) => val.to_string(),
            CardRank::Jack => "Jack".to_string(),
            CardRank::Queen => "Queen".to_string(),
            CardRank::King => "King".to_string(),
        }
    }
}

impl fmt::Display for PlayingCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_value())
    }
}
