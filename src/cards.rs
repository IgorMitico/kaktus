use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use rand::rng;
use rand::seq::SliceRandom;

use std::fmt;

use crate::player::Hand;

#[derive(EnumIter, Clone, Copy, Debug, PartialEq)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
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

pub struct Cards(pub Vec<Box<Card>>);

impl fmt::Display for Cards {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write(f, "\n")?;

        let mut index: u8 = 1;

        for card in &self.0 {
            let suit_emoji: &str = {
                match card.suit {
                    Suit::Clubs => "♣️",
                    Suit::Spades => "♠️",
                    Suit::Diamonds => "♦️",
                    Suit::Hearts => "♥️",
                }
            };
            let name_char: char = {
                match card.name {
                    CardName::King => 'K',
                    CardName::Jack => 'J',
                    CardName::Queen => 'Q',
                    _ => char::from_digit(card.value as u32, 10).expect("WTF"),
                }
            };
            write!(f, "{name_char}{suit_emoji}, ")?;

            if index % 10 == 0 {
                write!(f, "\n")?;
            }
            index += 1;
        }

        writeln!(f, "")
    }
}

impl Cards {}

pub struct Deck {
    pub cards: Cards,
}

impl Deck {
    pub fn build() -> Self {
        let mut cards = Cards(Vec::new());
        for suit in Suit::iter() {
            for name in CardName::iter() {
                cards.0.push(Box::new(Card::build(name, suit)));
            }
        }
        Deck { cards }
    }
    pub fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.0.shuffle(&mut rng);
    }
    pub fn draw(&mut self, hand: &mut Hand) {
        if let Some(x) = self.cards.0.pop() {
            hand.cards.0.push(x);
        }
    }
    // TODO: find a name for the function.
    // fonction that put the discarded cards in the drawl pile and shuffle them
    pub fn transfer_shuffle(&mut self, other: &mut Self) {
        while let Some(x) = self.cards.0.pop() {
            other.cards.0.push(x);
        }
        other.shuffle();
    }
}
