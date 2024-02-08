// Rusty Solitare - a solitare implementation written by Kevin in Rust.
// Will not have a GUI to start, but I may add one some day.

use std::io;

use rand::seq::SliceRandom;

#[derive(Debug)]
enum Number {
    K,
    Q,
    J,
    A,
    Value(u8)
}

#[derive(Debug, PartialEq)]
enum Suit {
    Diamond,
    Club,
    Heart,
    Spade
}

#[derive(Debug, PartialEq)]
enum Face {
    FaceUp,
    FaceDown,
}

#[derive(Debug)]
struct Card {
    number: Number,
    suit: Suit,
    // Note: This should probably be a new struct like PlayedCard, but I just want to do it this way for now at least..
    face: Option<Face>,
}

impl Card {
    fn new(number: u8, suit: Suit) -> Card {
        Card{
            number: Number::Value(number),
            suit: suit,
            face: Default::default(),
        }
    }
    
    fn is_black(&self) -> bool {
        return self.suit == Suit::Club || self.suit == Suit::Spade
    }
    
    fn repr_fixed(&self) -> String {
        // TODO: return fixed width so it's always 3 characters right aligned..
        // Output looks like..
        // " QH"
        // "10C"
        // " XX" for facedown card
        let mut s = String::new();
        
        // TODO if self.face == None panic!
        
        if self.face == Some(Face::FaceDown) {
            s.push_str(" XX");
            return s
        }
        
        let num;
        
        match self.number {
            Number::Value(v) => num = v,
            _ => num = 255 // TODO: JQKA etc...
        }
        
        let suit = match self.suit {
            Suit::Diamond => "♦",
            Suit::Club => "♣",
            Suit::Heart => "♥",
            Suit::Spade => "♠",
        };
        
        if num < 10 {
            s.push_str(" ");
        }
        
        
        s.push_str(&num.to_string());
        s.push_str(suit);
        return s;
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
    
        for i in 1..=13 {
            for j in 1..=4 {
                
                let suit = match j {
                    1 => Suit::Diamond,
                    2 => Suit::Club,
                    3 => Suit::Heart,
                    4 => Suit::Spade,
                    _ => panic!("Cannot match suit"),
                };
                
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
    
    fn is_empty(&self) -> bool {
        return self.cards.is_empty()
    }
    
    fn draw_card(&mut self) -> Card {
        if self.is_empty() {
            panic!("Trying to draw from empty deck");
        }
        self.cards.pop().unwrap()
    }
}

#[derive(Debug)]
struct GameBoard {
    card_piles: [Vec<Card>; 7],
    ace_piles: [Vec<Card>; 4],
    hand: Vec<Card>,
    discard: Vec<Card>,
}

impl GameBoard {
    fn new(deck: &mut Deck) -> GameBoard{
        let mut card_piles: [Vec<Card>; 7] = Default::default();
        let ace_piles: [Vec<Card>; 4] = Default::default();
        let hand = Vec::new();
        let discard = Vec::new();
        
        for i in 0..=6 {
            for j in 0..=6 {
                if j >= i {
                    let mut card = deck.draw_card();
                    if i == j {
                        card.face = Some(Face::FaceUp);
                    }
                    else if j > i {
                        card.face = Some(Face::FaceDown);
                    }
                    card_piles[j].push(card);
                }
            }
        }
        
        GameBoard{
            card_piles,
            ace_piles,
            hand,
            discard,
        }
    }
    
    fn print(&self) -> () {
        println!("      1   2   3   4   5   6   7");
        for num in 0..self.card_piles.len() {
            let c = (num + 97) as u8 as char;
            print!("[ {}]", c);
            let card_pile = &self.card_piles[num];
            for card in card_pile.iter() {
                print!(" {}", card.repr_fixed());
            }
            print!("\n");
        }
        
        println!("");
        for num in 0..self.ace_piles.len() {
            let c = (num + 65) as u8 as char;
            
            let suit = match c {
                'A' => "♦",
                'B' => "♣",
                'C' => "♥",
                'D' => "♠",
                _ => panic!("Invalid suit")
            };
            println!("[{}{}]", c, suit);
        }
    }
}


fn welcome() {
    println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@");
    println!("@ Welcome to Rusty Solitare! @");
    println!("@        By Kevin            @");
    println!("@  Please kindly ignore all  @");
    println!("@  those compiler warnings   @");
    println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@");
    println!("");
    println!("Press enter to start");
    print!(">>>");
    println!("");
    let mut input = String::new();
    io::stdin().read_line(&mut input);
}

fn main() {
    // Since we know length we could use an array here an it'd be much much speedier.
    
    welcome();
    
    let mut deck = Deck::new();
    deck.shuffle();
    
    let mut game_board = GameBoard::new(&mut deck);
    
    //dbg!(&game_board);
    
    clearscreen::clear().expect("failed to clear screen");

    game_board.print();
    
    
    //dbg!(&card);
    //dbg!(&deck);
    //dbg!(&deck.cards[0]);

}
