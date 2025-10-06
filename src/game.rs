use crate::cards;
use crate::player;

pub fn start(players: &mut player::Players, deck: &mut cards::Deck) {
    for n in 0..players.n_players {
        for _ in 0..4 {
            deck.draw(&mut players.players[n].hand);
        }
    }
}
