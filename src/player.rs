use crate::cards::Cards;

pub struct Hand {
    pub cards: Cards,
}

impl Hand {
    pub fn init() -> Self {
        let cards = Cards(Vec::new());
        Hand { cards }
    }
    pub fn get_value(&self) -> u8 {
        self.cards.0.iter().map(|c| c.value).sum::<u8>()
    }
}

pub struct Player {
    pub hand: Hand,
    pub name: String,
}
