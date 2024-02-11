// Rusty Solitare - a solitare implementation written by Kevin in Rust.
// Will not have a GUI to start, but I may add one some day.

use std::io;
use std::io::Error;

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
enum Colour {
    Red,
    Black,
}

#[derive(Debug, PartialEq)]
enum Face {
    FaceUp,
    FaceDown,
}

#[derive(Debug)]
struct Card {
    number: u8,  // TODO: Consider using Number enum again but it makes math really hard...
    suit: Suit,
    // Note: This should probably be a new struct like PlayedCard, but I just want to do it this way for now at least..
    face: Option<Face>,
}

impl Card {
    fn new(number: u8, suit: Suit) -> Card {
        Card{
            number: number,
            suit: suit,
            face: Default::default(),
        }
    }

    fn colour(&self) -> Colour {
        if self.suit == Suit::Diamond || self.suit == Suit::Heart {
            return Colour::Red;
        }
        Colour::Black
    }

    fn get_face_value(&self) -> String {
        let mut out = String::new();
        if self.number == 1 {
            out = "A".to_string();
        }
        else if self.number == 11 {
            out = "J".to_string();
        }
        else if self.number == 12 {
            out = "Q".to_string();
        }
        else if self.number == 13 {
            out = "K".to_string();
        }
        else {
            out = self.number.to_string();
        }

        return out
    }

    fn repr_fixed(&self) -> String {
        // TODO: return fixed width so it's always 3 characters right aligned..
        // Output looks like..
        // " QH"
        // "10C"
        // " XX" for facedown card
        let mut s: String = String::new();
        
        // TODO if self.face == None panic!
        
        if self.face == Some(Face::FaceDown) {
            s.push_str(" XX");
            return s
        }

        let suit = match self.suit {
            Suit::Diamond => "♦",
            Suit::Club => "♣",
            Suit::Heart => "♥",
            Suit::Spade => "♠",
        };
        
        if self.number != 10 {
            s.push_str(" ");
        }
        
        if self.colour() == Colour::Red {
            s.push_str("\x1b[31m");
            s.push_str(&self.get_face_value().to_string());
            s.push_str(suit);
            s.push_str("\x1b[0m");
        }
        else {
            s.push_str(&self.get_face_value().to_string());
            s.push_str(suit);
        }

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
        let mut hand = Vec::new();
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
        
        while !deck.is_empty() {
            hand.push(deck.draw_card());
        }
        
        GameBoard{
            card_piles,
            ace_piles,
            hand,
            discard,
        }
    }
    
    fn draw_cards(&mut self) {
        let draw_size = 3;  // number of cards to draw per turn TODO make static variable
        
        //panic!("{} {}", self.hand.len(), draw_size);
        
        if self.hand.len() >= draw_size {
            for i in 0..draw_size {
                self.discard.push(self.hand.pop().unwrap());
            }
        }
        else if self.hand.len() >= 1 {
            for i in 0..self.hand.len() {
                self.discard.push(self.hand.pop().unwrap());
            }
        }
        else {
            for i in 0..self.discard.len() {
                self.hand.push(self.discard.pop().unwrap());
            }
        }
    }

    fn place_card(&mut self, command: &str) -> Result<(), &'static str> {
        // Take a command which is just a single letter and try to place the top discard
        // card into the corresponding card_pile.
        let card_pile_char = command.chars().collect::<Vec<char>>()[0];

        if self.discard.len() == 0 {
            return Err("Cannot place card when discard is empty.")
        }

        let mut card_pile = &mut self.card_piles[card_pile_char as usize - 97];

        // TODO: If empty then allow placing King only.
        let top_card_pile = card_pile.last().unwrap();
        let top_discard = self.discard.last().unwrap();

        // If the colour is different and the number is 1 smaller than the card can be placed,
        // otherwise it's not allowed.
        if top_discard.number == top_card_pile.number - 1 && top_discard.colour() != top_card_pile.colour() {
            card_pile.push(self.discard.pop().unwrap());
        }
        else {
            return Err("Cannot move card onto that card pile!");
        }

        Ok(())

    }
    
    fn print(&self) -> () {
        
        // Print table card piles
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
        
        // Print ace card piles
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
        
        // Print deck
        println!("");
        println!("deck: {}", self.hand.len());
        
        // Print discard
        println!("");
        if self.discard.len() >= 1 {
            let top_card = self.discard.last().unwrap();
            println!("{} {}", self.discard.len(), top_card.repr_fixed());
        }
        else {
            println!("{}", self.discard.len());
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

fn handle_turn(game_board: &mut GameBoard, input: String) -> i32 {
    // On a turn can either draw cards, place a card from the discard, or move cards on the table.
    // Or quit.

    let command = input.trim();
    if command == "quit" {
        return 1
    }
    else if command == "" {
        game_board.draw_cards();
    }
    else if command.len() == 1 {
        game_board.place_card(command);
    }

    return 0
}

fn main() {
    // Since we know length we could use an array here an it'd be much much speedier.
    
    welcome();
    
    let mut deck = Deck::new();
    deck.shuffle();
    
    let mut game_board = GameBoard::new(&mut deck);
    
    //dbg!(&game_board);
    
    loop {
        clearscreen::clear().expect("failed to clear screen");
        game_board.print();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input);

        match handle_turn(&mut game_board, input) {
            1 => break,
            0 => continue,
            _ => panic!("bad response"),
        };
    }
   
    //dbg!(&card);
    //dbg!(&deck);
    //dbg!(&deck.cards[0]);

}
