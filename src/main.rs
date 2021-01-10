#![allow(dead_code)]

//! Super Card Game

use rand::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::{thread, time};

mod cards;

use cards::card_printer::display_hand;
use cards::{Card, DeckBuilder};

/// GameBuilder struct representing game options.
struct GameBuilder {
    card_count: u8,
}

/// Builds the game object using the builder pattern.
impl GameBuilder {
    /// GameBuilder Contsturctor.
    fn new() -> GameBuilder {
        GameBuilder { card_count: 3 }
    }

    /// Option to change the number of players.
    fn max_cards(mut self, count: u8) -> GameBuilder {
        self.card_count = match count {
            0..=2 => panic!("Must have more than one card."),
            3..=5 => count,
            _ => panic!("Too many cards."),
        };
        self
    }

    /// Creates a new Game opject.
    fn spawn(self) -> Game {
        Game {
            deck: DeckBuilder::new(),
            cards: Vec::new(),
            games_played: 0,
            wins: 0,
        }
    }
}

/// Holds game state.
struct Game {
    deck: Vec<Card>,
    cards: Vec<Card>,
    games_played: usize,
    wins: usize,
}

impl Game {
    /// Randomly shuffles cards
    fn shuffle_deck(&mut self) {
        let mut rng = thread_rng();
        self.deck.shuffle(&mut rng);
    }

    fn deal_cards(&mut self) {
        self.shuffle_deck();
        self.cards = self.deck.drain(0..3).collect();
    }

    fn find_high_card(&self) -> usize {
        let mut index = 0;
        let mut value = 0;
        for (idx, card) in self.cards.iter().enumerate() {
            if card.value() > value {
                value = card.value();
                index = idx;
            }
        }
        index
    }

    fn inc_gamesplayed(&mut self) {
        self.games_played += 1;
    }

    fn inc_wins(&mut self) {
        self.wins += 1;
    }

    fn out_of_cards(&self) -> bool {
        if self.deck.len() < 3 {
            return true;
        }
        false
    }
}

use std::fmt::{self, Display, Formatter};
use std::io::{self};

impl Display for Game {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(
            formatter,
            "Won {} out of {} games.\nCards Left {}",
            self.wins,
            self.games_played,
            self.deck.len(),
        )
    }
}

fn main() {
    let mut game: Game = GameBuilder::new().spawn();
    println!("{}", game.cards.len());

    while !game.out_of_cards() {
        game.deal_cards();

        let winning_card = game.find_high_card();
        let sleep_time = time::Duration::from_secs(1);

        display_hand(&game.cards, true);

        println!("Find the High card.");
        println!("Press [Enter] for a random choice.");

        let mut input = String::new();
        let mut choice: usize = rand::thread_rng().gen_range(0..2);

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if let Ok(i) = input.trim().parse::<usize>() {
                    if i > game.cards.len() - 1 {
                        choice = game.cards.len() - 1;
                    } else {
                        choice = i;
                    }
                }
            }
            Err(_) => {}
        }

        game.cards[choice].toggle();
        display_hand(&game.cards, true);

        println!("Lets see the results.");

        thread::sleep(sleep_time);

        game.cards[choice].toggle();
        game.cards[0].toggle();
        game.cards[1].toggle();
        game.cards[2].toggle();

        display_hand(&game.cards, true);

        if choice == winning_card {
            game.inc_wins();
            println!("You win!!!")
        } else {
            println!("You lose!")
        }

        game.inc_gamesplayed();
        println!("{}\n\n", game);

        thread::sleep(sleep_time);
    }

    println!("Sorry ran out of cards.");
}
