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
}

pub struct Player {
    pub hand: Hand,
    pub name: String,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n{}", &self.name, &self.hand.cards)
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
}

pub struct Players {
    pub players: Vec<Player>,
    pub n_players: usize,
}

impl fmt::Display for Players {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for n in 0..self.n_players {
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
        Players { players, n_players }
    }
}
