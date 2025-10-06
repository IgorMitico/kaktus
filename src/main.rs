use kaktus::cards;
use kaktus::game;
use kaktus::player;

fn main() {
    let mut draw_pile = cards::Deck::build();
    let mut players = player::Players::init(4);

    println!("\nDECK:\n{}", draw_pile.cards);
    draw_pile.shuffle();
    println!("\nAFTER THE SHUFFLE\n\nDECK:\n{}", draw_pile.cards);

    game::start(&mut players, &mut draw_pile);

    println!("\nDECK:\n{}\n\n{}", draw_pile.cards, players);
}
