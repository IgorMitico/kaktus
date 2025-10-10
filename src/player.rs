use crate::cards;
use crate::cards::Cards;
use std::fmt;

pub const MAX_PLAYERS: usize = 6;

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
    pub fn contains(&self, card_name: cards::CardName, card_suit: cards::Suit) -> bool {
        let mut flag = false;
        for card in &self.cards.0 {
            if card.name == card_name && card.suit == card_suit {
                flag = true;
            }
        }
        flag
    }
}

pub struct Player {
    pub hand: Hand,
    pub name: String,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\n{}\n",
            &self.name,
            &self.hand.cards,
        )
    }
}

impl Player {
    pub fn init() -> Self {
        let player = Player {
            hand: Hand::init(),
            name: String::from(""),
        };
        player
    }
    pub fn discard(&mut self, card: usize, pile: &mut cards::Deck) {
        pile.cards.0.push(self.hand.cards.0.remove(card));
    }
    pub fn draw(&mut self, pile: &mut cards::Deck) {
        if let Some(x) = pile.cards.0.pop() {
            self.hand.cards.0.push(x);
        }
    }
    // we need to find a name
    pub fn the_move(&mut self, card1: usize, card2: usize, draw_pile: &mut cards::Deck, discard_pile: &mut cards::Deck) {
        if card1 < self.hand.cards.0.len() && card2 < self.hand.cards.0.len() {
            if self.hand.cards.0[card1].name == self.hand.cards.0[card2].name {
                self.draw(discard_pile);
                self.discard(card1, discard_pile);
                self.discard(card2, discard_pile);
            }
            else {
                self.draw(draw_pile);
            }
        }
    }
}

pub struct Players {
    pub players: Vec<Player>,
}

impl fmt::Display for Players {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for n in 0..self.players.len() {
            write!(f, "{}", &self.players[n])?;
        }
        write!(f, "--------------------")
    }
}

impl Players {
    pub fn init(n_players: usize) -> Self {
        let mut players = Vec::new();
        for n in 0..n_players {
            players.push(Player {
                hand: Hand::init(),
                name: format!("player_{n}"),
            });
        }
        Players { players }
    }
}
