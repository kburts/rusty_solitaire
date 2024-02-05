// Rusty Solitare - a solitare implementation written by Kevin in Rust.
// Will not have a GUI to start, but I may add one some day.

use rand::seq::SliceRandom;

#[derive(Debug)]
enum Number {
    Value(u8)
}

#[derive(Debug)]
enum Suit {
    Diamond,
    Club,
    Heart,
    Spade
}

#[derive(Debug)]
struct Card {
    number: Number,
    suit: Suit
}

impl Card {
    fn new(number: u8, suit: Suit) -> Card {
        Card{
            number: Number::Value(number),
            suit: suit
        }
    }
}

#[derive(Debug)]
struct Deck {
    cards: Vec<Card>
}

impl Deck {
    fn new() -> Deck {
        let cards = Vec::new();
        let mut deck = Deck{cards};
    
        for i in 1..=12 {
            for j in 1..=4 {
                
                let suit;
                
                match j {
                    1 => suit = Suit::Diamond,
                    2 => suit = Suit::Club,
                    3 => suit = Suit::Heart,
                    4 => suit = Suit::Spade,
                    _ => panic!("Cannot match suit"),
                }
                
                deck.cards.push(
                    Card::new(
                        i,
                        suit,
                    )
                )
            }
        }
        deck
    }
    
    fn shuffle(&mut self) -> () {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }
}


fn main() {
    println!("Hello, world!");

    // Since we know length we could use an array here an it'd be much much speedier.
    let mut deck = Deck::new();
    deck.shuffle();
    
    
    
    //dbg!(&card);
    //dbg!(&deck);
    dbg!(&deck.cards[0]);
}
