use kaktus::cards;
use kaktus::game;
use kaktus::player;

fn main() {
    let mut draw_pile = cards::Deck::init();
    let mut discard_pile = cards::Deck::init();
    let mut players = player::Players::init(4);

    game::start(&mut players, &mut draw_pile, &mut discard_pile);

    println!("\nDECK:\n{}\nDISCARD PILE:\n{}\n\n{}", draw_pile.cards, discard_pile.cards, players);

    let (winner, value) = game::end(&players, 3);

    println!("The winner is: {} with {} points in hand", players.players[winner].name, value);
}
