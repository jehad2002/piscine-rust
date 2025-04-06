use rand::Rng;

#[derive(Debug, PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, PartialEq)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

#[derive(Debug, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn generate_suit() -> Suit {
    let value = rand::thread_rng().gen_range(1..=4);
    match value {
        1 => Suit::Heart,
        2 => Suit::Diamond,
        3 => Suit::Spade,
        _ => Suit::Club,
    }
}

pub fn generate_rank() -> Rank {
    let value = rand::thread_rng().gen_range(1..=13);
    match value {
        1 => Rank::Ace,
        11 => Rank::Jack,
        12 => Rank::Queen,
        13 => Rank::King,
        _ => Rank::Number(value),
    }
}

pub fn generate_card() -> Card {
    Card {
        suit: generate_suit(),
        rank: generate_rank(),
    }
}

pub fn winner_card(card: &Card) -> bool {
    matches!(card.rank, Rank::Ace) && matches!(card.suit, Suit::Spade)
}
