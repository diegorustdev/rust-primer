#[derive(Debug)]

struct Deck {
    cards: Vec<String>,
}

fn main() {
    
    let suits = vec!["Hearts", "Spades", "Diamonds"];
    let values = vec!["Ace", "Two", "Three"];

    let deck = Deck { cards: vec![] };

    println!("Heres your deck: {:?}", deck);
}
