use ansi_term::Colour;
/// This module provides abstractions and methods for building and interacting with a Standard 52-card deck.
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
            Suit::Spades => Colour::White.paint("♠"),
            Suit::Diamonds => Colour::Red.paint("♦"),
            Suit::Hearts => Colour::Red.paint("♥"),
            Suit::Clubs => Colour::White.paint("♣"),
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

/// Static helper for easy and cheap iteration over suits.
static SUITS: [Suit; 4] = [Suit::Spades, Suit::Diamonds, Suit::Hearts, Suit::Clubs];

/// Static helper for easy and cheap iteration over ranks.
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CardState {
    Visible,
    Hidden,
}

/// Represets a single card with a suit and rank.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
    pub state: CardState,
}

impl Card {
    pub fn toggle(&mut self) {
        match self.state {
            CardState::Visible => self.state = CardState::Hidden,
            CardState::Hidden => self.state = CardState::Visible,
        }
    }
}

impl Card {
    /// Construct a new card struct
    ///
    pub fn new(suit: Suit, rank: Rank) -> Card {
        Card {
            suit,
            rank,
            state: CardState::Hidden,
        }
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

/// This module provides functions for creating displayable cards.
///
/// Example:
/// ```text
/// *---------* *---------* *---------*
/// | K       | | 4       | | 8       |
/// |         | |         | |         |
/// |    ♠    | |    ♥    | |    ♣    |
/// |         | |         | |         |
/// |       K | |       4 | |       8 |
/// *---------* *---------* *---------*
/// ```
///
pub mod card_printer {
    use super::{Card, CardState, Rank};

    fn print_end(hand: &Vec<Card>) {
        for _ in 0..hand.len() {
            print!("*---------*");
            print!(" ")
        }
        println!();
    }

    fn print_empty_section(hand: &Vec<Card>) {
        for card in hand {
            match card.state {
                CardState::Hidden => {
                    print!("|#########|");
                    print!(" ")
                }
                CardState::Visible => {
                    print!("|         |");
                    print!(" ")
                }
            }
        }
        println!();
    }

    fn print_left_rank(hand: &Vec<Card>) {
        for card in hand {
            match card.state {
                CardState::Hidden => {
                    print!("|#########|");
                    print!(" ")
                }

                CardState::Visible => {
                    if let Rank::Ten = card.rank {
                        print!("| {}      |", card.rank);
                    } else {
                        print!("| {}       |", card.rank);
                    }
                    print!(" ")
                }
            }
        }
        println!();
    }

    fn print_right_rank(hand: &Vec<Card>) {
        for card in hand {
            match card.state {
                CardState::Hidden => {
                    print!("|#########|");
                    print!(" ")
                }
                CardState::Visible => {
                    if let Rank::Ten = card.rank {
                        print!("|      {} |", card.rank);
                    } else {
                        print!("|       {} |", card.rank);
                    }
                    print!(" ")
                }
            }
        }
        println!();
    }

    fn print_suit(hand: &Vec<Card>) {
        for card in hand {
            match card.state {
                CardState::Hidden => {
                    print!("|#########|");
                    print!(" ")
                }

                CardState::Visible => {
                    print!("|    {}    |", card.suit);
                    print!(" ")
                }
            }
        }
        println!();
    }

    fn print_index(hand: &Vec<Card>) {
        for (idx, card) in hand.iter().enumerate() {
            print!("     {}     ", idx);
            print!(" ")
        }
        println!();
    }

    pub fn display_hand(hand: &Vec<Card>, show_index: bool) {
        print_end(&hand);
        print_left_rank(&hand);
        print_empty_section(&hand);
        print_suit(&hand);
        print_empty_section(&hand);
        print_right_rank(&hand);
        print_end(&hand);

        if show_index {
            print_index(&hand);
        }
    }
}
