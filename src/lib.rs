use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use rand::rng;
use rand::seq::SliceRandom;

#[derive(EnumIter, Clone, Copy, Debug)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(EnumIter, Clone, Copy, Debug, PartialEq)]
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
    pub fn get_value(&self) -> u8 {
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
    pub value: u8,
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
    pub cards: Vec<Box<Card>>,
}

impl Deck {
    pub fn build() -> Self {
        let mut cards: Vec<Box<Card>> = Vec::new();
        for suit in Suit::iter() {
            for name in CardName::iter() {
                cards.push(Box::new(Card::build(name, suit)));
            }
        }
        Deck { cards }
    }
    pub fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }
    pub fn draw(&mut self, hand: &mut Hand) {
        if let Some(x) = self.cards.pop() {
            hand.cards.push(x);
        }
    }
    pub fn print(&self) {
        for card in &self.cards {
            println!("{:?} of {:?}", card.name, card.suit);
        }
    }
    // TODO: find a name for the function.
    // fonction that put the discarded cards in the drawl pile and shuffle them
    pub fn transfer_shuffle(&mut self, other: &mut Self) {
        while let Some(x) = self.cards.pop() {
            other.cards.push(x);
        }
        other.shuffle();
    }
}

pub struct Hand {
    pub cards: Vec<Box<Card>>,
}

impl Hand {
    pub fn print(&self) {
        for card in &self.cards {
            println!("{:?} of {:?}", card.name, card.suit);
        }
    }
    pub fn get_value(&self) -> u8 {
        self.cards.iter().map(|c| c.value).sum::<u8>()
    }
}
