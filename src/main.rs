//! Super Card Game

use rand::seq::SliceRandom;
use rand::thread_rng;

/// This module provides abstractions and methods for building and interacting with a Standard 52-card deck.
mod cards {

    use std::fmt::{self, Display, Formatter};

    /// Represent Card Suits
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Suit {
        Spades = 1,
        Diamonds = 2,
        Hearts = 3,
        Clubs = 4,
    }

    /// Displays Card Suits with symbols.
    impl Display for Suit {
        fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
            let s = match self {
                Suit::Spades => "♠",
                Suit::Diamonds => "♦",
                Suit::Hearts => "♥",
                Suit::Clubs => "♣",
            };
            write!(formatter, "{}", s)
        }
    }

    /// Represent Card Ranks
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Rank {
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        Jack,
        Queen,
        King,
        Ace,
    }

    /// Convert Rank to integer values.
    impl Rank {
        fn value(&self) -> u32 {
            match self {
                Rank::Two => 2,
                Rank::Three => 3,
                Rank::Four => 4,
                Rank::Five => 5,
                Rank::Six => 6,
                Rank::Seven => 7,
                Rank::Eight => 8,
                Rank::Nine => 9,
                Rank::Ten => 10,
                Rank::Jack => 10,
                Rank::Queen => 10,
                Rank::King => 10,
                Rank::Ace => 11,
            }
        }
    }

    /// String representation of each rank.
    impl Display for Rank {
        fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
            let s = match self {
                Rank::Two => "2",
                Rank::Three => "3",
                Rank::Four => "4",
                Rank::Five => "5",
                Rank::Six => "6",
                Rank::Seven => "7",
                Rank::Eight => "8",
                Rank::Nine => "9",
                Rank::Ten => "10",
                Rank::Jack => "J",
                Rank::Queen => "Q",
                Rank::King => "K",
                Rank::Ace => "A",
            };
            write!(formatter, "{}", s)
        }
    }

    /// Static helpers for easy and cheap iteration to create a deck of cards.
    static SUITS: [Suit; 4] = [Suit::Spades, Suit::Diamonds, Suit::Hearts, Suit::Clubs];
    static RANKS: [Rank; 13] = [
        Rank::Two,
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
        Rank::Nine,
        Rank::Ten,
        Rank::Jack,
        Rank::Queen,
        Rank::King,
        Rank::Ace,
    ];

    /// Represets a single card with a suit and rank.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Card {
        pub suit: Suit,
        pub rank: Rank,
    }

    impl Card {
        /// Construct a new card struct
        ///
        pub fn new(suit: Suit, rank: Rank) -> Card {
            Card { suit, rank }
        }

        /// Displays card nomenclature
        ///
        /// Example:
        /// Ace of Spades
        ///
        pub fn nomenclature(&self) -> String {
            format!("{:?}\tof {:?}\t", self.rank, self.suit)
        }

        /// Value of card.
        /// Value is calculated by suit point number * number in the card.
        ///
        pub fn value(&self) -> u32 {
            (self.suit as u32) * self.rank.value()
        }
    }

    /// DeckBuilder is used to create a new Deck of Cards.
    ///
    pub struct DeckBuilder;

    impl DeckBuilder {
        pub fn new() -> Vec<Card> {
            let mut deck: Vec<Card> = vec![];

            for rank in RANKS.iter() {
                for suit in SUITS.iter() {
                    deck.push(Card::new(*suit, *rank))
                }
            }

            deck
        }
    }

    #[cfg(test)]
    mod tests {
        use super::{Card, DeckBuilder, Rank, Suit};
        use insta;

        #[test]
        /// Tests sorting.
        fn test_deck_sorts() {
            //(Spades, 2), (Diamonds, 5), (Spades, King), (Hearts, 3), (Clubs, Ace)
            let mut unsorted_deck: Vec<Card> = Vec::new();
            unsorted_deck.push(Card::new(Suit::Spades, Rank::Two));
            unsorted_deck.push(Card::new(Suit::Diamonds, Rank::Five));
            unsorted_deck.push(Card::new(Suit::Spades, Rank::King));
            unsorted_deck.push(Card::new(Suit::Hearts, Rank::Three));
            unsorted_deck.push(Card::new(Suit::Clubs, Rank::Ace));

            unsorted_deck.sort();

            //(Spades, 2), (Spades, King), (Diamonds, 5), (Hearts, 3), (Clubs, Ace)
            let mut sorted_deck: Vec<Card> = Vec::new();
            sorted_deck.push(Card::new(Suit::Spades, Rank::Two));
            sorted_deck.push(Card::new(Suit::Spades, Rank::King));
            sorted_deck.push(Card::new(Suit::Diamonds, Rank::Five));
            sorted_deck.push(Card::new(Suit::Hearts, Rank::Three));
            sorted_deck.push(Card::new(Suit::Clubs, Rank::Ace));

            assert_eq!(unsorted_deck, sorted_deck);
        }

        #[test]
        /// Tests 52 cards are created.
        fn test_deck_builder_length() {
            let test_deck = DeckBuilder::new();
            assert_eq!(test_deck.len(), 52);
        }

        #[test]
        /// Tests nomenclatures of each card
        fn test_nomenclature() {
            let nomenclatures: Vec<_> = DeckBuilder::new()
                .iter()
                .map(|card| card.nomenclature())
                .collect();

            insta::assert_debug_snapshot!(nomenclatures);
        }

        #[test]
        /// Tests card values
        fn test_card_value() {
            let values: Vec<_> = DeckBuilder::new().iter().map(|card| card.value()).collect();

            insta::assert_debug_snapshot!(values);
        }

        #[test]
        /// Tests card displays
        fn test_rank_suit_display() {
            let cards: Vec<_> = DeckBuilder::new()
                .iter()
                .map(|card| format!("{} {}", card.suit, card.rank))
                .collect();
            insta::assert_debug_snapshot!(cards);
        }
    }
}

/// This module provides functions for creating displayable cards.
///
/// Example:
/// *---------* *---------* *---------*
/// | K       | | 4       | | 8       |
/// |         | |         | |         |
/// |    ♠    | |    ♥    | |    ♣    |
/// |         | |         | |         |
/// |       K | |       4 | |       8 |
/// *---------* *---------* *---------*
///
mod card_printer {
    use super::cards::{Card, Rank};

    fn print_end(size: usize) {
        for _ in 0..size {
            print!("*---------*");
            print!(" ")
        }
        println!();
    }

    fn print_empty_section(size: usize) {
        for _ in 0..size {
            print!("|         |");
            print!(" ")
        }
        println!();
    }

    fn print_left_rank(deck: &Vec<Card>) {
        for card in deck {
            if let Rank::Ten = card.rank {
                print!("| {}      |", card.rank);
            } else {
                print!("| {}       |", card.rank);
            }
            print!(" ")
        }
        println!();
    }

    fn print_right_rank(deck: &Vec<Card>) {
        for card in deck {
            if let Rank::Ten = card.rank {
                print!("|      {} |", card.rank);
            } else {
                print!("|       {} |", card.rank);
            }
            print!(" ")
        }
        println!();
    }

    fn print_suit(deck: &Vec<Card>) {
        for card in deck {
            print!("|    {}    |", card.suit);
            print!(" ")
        }
        println!();
    }

    pub fn display_hand(deck: &Vec<Card>, name: &str) {
        println!("{: >width$}'s Hand", name, width = (deck.len() * 12) / 2); // todo: move
        let width = deck.len();

        print_end(width);
        print_left_rank(&deck);
        print_empty_section(width);
        print_suit(&deck);
        print_empty_section(width);
        print_right_rank(&deck);
        print_end(width);
    }
}

use card_printer::display_hand;
use cards::{Card, DeckBuilder};

/// GameBuilder struct representing game options.
struct GameBuilder {
    player_count: u8,
}

/// Builds the game object using the builder pattern.
impl GameBuilder {
    /// GameBuilder Contsturctor.
    fn new() -> GameBuilder {
        GameBuilder { player_count: 2 }
    }

    /// Option to change the number of players.
    fn players(mut self, count: u8) -> GameBuilder {
        self.player_count = match count {
            0..=1 => panic!("Must have more than 1 player."),
            2..=5 => count,
            _ => panic!("Too many players."),
        };
        self
    }

    /// Creates a new Game opject.
    fn spawn(self) -> Game {
        Game {
            deck: DeckBuilder::new(),

            players: {
                let mut players = vec![];
                for _ in 0..self.player_count {
                    players.push(Vec::new());
                }
                players
            },

            turn: 0,
        }
    }
}

/// Scores a players hand of cards.
fn score_hand(hand: &Vec<Card>) -> u32 {
    hand.iter().fold(0, |sum, card| sum + card.value())
}

/// Holds game state.
struct Game {
    deck: Vec<Card>,
    players: Vec<Vec<Card>>,
    turn: usize,
}

impl Game {
    /// Randomly shuffles cards
    fn shuffle_deck(&mut self) {
        let mut rng = thread_rng();
        self.deck.shuffle(&mut rng);
    }

    /// Checks the state of the game.
    /// The game is complete when all players have 3 cards.
    fn is_over(&self) -> bool {
        self.players.iter().all(|card| card.len() > 2)
    }

    /// Calculate all the players cards and prints the winner.
    fn show_winner(&self) {
        let mut winner = 0;
        let mut winning_score = 0;

        for (player, hand) in self.players.iter().enumerate() {
            let score = score_hand(&hand);

            if score > winning_score {
                winning_score = score;
                winner = player
            }
        }
        println!("Player {} won!!!!", winner);
    }

    /// Shows the scores and hands of each player.
    fn show_results(&self) {
        for (player, hand) in self.players.iter().enumerate() {
            let name = format!("Player {}", player);
            display_hand(&hand, &name);

            let score = score_hand(&hand);
            println!("SCORE: {}", score);
        }
    }

    /// Increments the game forward.
    fn advance(&mut self) {
        let card = self.deck.pop().unwrap();
        let player = self.turn % self.players.len();

        println!(
            "Player {} picked a {} with a value of {}.",
            player,
            card.nomenclature(),
            card.value()
        );

        self.players[player].push(card);
        self.players[player].sort();

        self.turn += 1;
    }
}

fn main() {
    let mut game: Game = GameBuilder::new().players(3).spawn();

    game.shuffle_deck();

    while !game.is_over() {
        game.advance();
    }

    game.show_winner();

    game.show_results();
}
