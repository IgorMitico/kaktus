use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Clone, Copy)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(EnumIter, Clone, Copy)]
pub enum CardName {
    King,
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Jack,
    Queen,
}

impl CardName {
    pub fn get_value(&self) -> usize {
        match self {
            CardName::King => 0,
            CardName::Ace => 1,
            CardName::Two => 2,
            CardName::Three => 3,
            CardName::Four => 4,
            CardName::Five => 5,
            CardName::Six => 6,
            CardName::Seven => 7,
            CardName::Jack => 10,
            CardName::Queen => 10,
        }
    }
}

pub struct Card {
    pub name: CardName,
    pub value: usize,
    pub suit: Suit,
}

impl Card {
    pub fn build(name: CardName, suit: Suit) -> Self {
        Card {
            name,
            value: CardName::get_value(&name),
            suit,
        }
    }
}

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn build() -> Self {
        let mut cards: Vec<Card> = Vec::new();
        for suit in Suit::iter() {
            for name in CardName::iter() {
                cards.push(Card::build(name, suit));
            }
        }
        Deck { cards }
    }
}
