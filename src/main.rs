use kaktus::player;
use kaktus::cards;

fn main() {
    let mut draw_pile = cards::Deck::build();

    println!("\nDECK:\n{}", draw_pile.cards);
    draw_pile.shuffle();
    println!("\nAFTER THE SHUFFLE\n\nDECK:\n{}", draw_pile.cards);

    let mut hand = player::Hand::init();

    for _ in 0..4 {
        draw_pile.draw(&mut hand);
    }

    println!("\nDECK:\n{} \nHAND:\n{}", draw_pile.cards, hand.cards);
    println!("\nThe value of the hand is {}", hand.get_value());
}
