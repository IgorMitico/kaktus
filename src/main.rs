use kaktus::Deck;
use kaktus::Hand;

fn main() {
    let mut draw_pile = Deck::build();

    println!("\nDECK: ");
    draw_pile.print();
    println!("\nAFTER THE SHUFFLE\n");
    draw_pile.shuffle();
    println!("\nDECK: ");
    draw_pile.print();

    let mut hand = Hand { cards: Vec::new() };

    for _ in 0..4 {
        draw_pile.draw(&mut hand);
    }

    println!("\nDECK: ");
    draw_pile.print();
    println!("\nHAND: ");
    hand.print();
    println!("\nThe value of the hand is {}", hand.get_value());
}
