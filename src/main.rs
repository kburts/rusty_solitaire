// Rusty Solitare - a solitare implementation written by Kevin in Rust.
// Will not have a GUI to start, but I may add one some day.

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
    
    fn is_black(&self) -> bool {
        return self.suit == Suit::Club || self.suit == Suit::Spade
    }
    
    fn repr_fixed(&self) -> String {
        // TODO: return fixed width so it's always 3 characters right aligned.
        let mut s = String::new();
        
        let num;
        
        match self.number {
            Number::Value(v) => num = v,
            _ => num = 255 // TODO: JQKA etc...
        }
        
        let suit;
        
        match self.suit {
            Suit::Diamond => suit = "D",
            Suit::Club => suit = "C",
            Suit::Heart => suit = "H",
            Suit::Spade => suit = "S",
        }
        
        if num < 10 {
            s.push_str(" ");
        }
        s.push_str(" ");
        
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
                if i == j {
                    // Face up card
                    card_piles[j].push(deck.draw_card());
                }
                else if j > i {
                    // face down cards
                    card_piles[j].push(deck.draw_card());
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
        for card_pile in self.card_piles.iter() {
            for card in card_pile.iter() {
                print!("{}", card.repr_fixed());
            }
            print!("\n");
        }
    }
}


fn main() {
    // Since we know length we could use an array here an it'd be much much speedier.
    let mut deck = Deck::new();
    deck.shuffle();
    
    let mut game_board = GameBoard::new(&mut deck);
    
    //dbg!(&game_board);
    game_board.print();
    
    
    //dbg!(&card);
    //dbg!(&deck);
    dbg!(&deck.cards[0]);
}
