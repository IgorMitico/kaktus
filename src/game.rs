use strum::IntoEnumIterator;

use crate::cards;
use crate::player;
use crate::player::Players;

pub fn start(players: &mut Players, draw_pile: &mut cards::Deck, discard_pile: &mut cards::Deck) {
    draw_pile.build();
    draw_pile.shuffle();
    for n in 0..players.players.len() {
        for _ in 0..4 {
            players.players[n].draw(draw_pile);
        }
    }
    if let Some(x) = draw_pile.cards.0.pop() {
        discard_pile.cards.0.push(x);
    }
}   

pub fn end(players: &Players, k_player: usize) -> (usize, u8) {
    let mut min_value = players.players[k_player].hand.get_value();
    let mut winner: usize = k_player;
    for n in 0..players.players.len() {
        if n == k_player {
            continue;
        }
        let n_value = players.players[n].hand.get_value();
        if n_value < min_value {
            min_value = n_value;
            winner = n;
        } else if min_value == n_value {
            if k_player != winner {
                if check_winner(&players.players[n].hand, &players.players[winner].hand) {
                    winner = n;
                }
            }
        }
    }
    (winner, min_value)
}

fn check_winner(p1: &player::Hand, p2: &player::Hand) -> bool {
    if p1.cards.0.len() < p2.cards.0.len() {
        return true;
    } else if p1.cards.0.len() > p2.cards.0.len() {
        return false;
    } else {
        for card in cards::CardName::iter() {
            if count(&p1.cards, card) > count(&p2.cards, card) {
                return true;
            } else if count(&p1.cards, card) < count(&p2.cards, card) {
                return false;
            }
        }
        for card_name in cards::CardName::iter() {
            for card_suit in cards::Suit::iter() {
                if p1.contains(card_name, card_suit) {
                    return true;
                } else if p2.contains(card_name, card_suit) {
                    return false;
                }
            }
        }
        return false;
    }
}

fn count(cards: &cards::Cards, card_name: cards::CardName) -> usize {
    let mut count: usize = 0;
    for card in &cards.0 {
        if card.name == card_name {
            count += 1;
        }
    }
    count
}
