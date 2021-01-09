//! Super Card Game

use rand::seq::SliceRandom;
use rand::thread_rng;

mod cards;

struct Player {
    hand: Vec<Card>,
}

impl Player {
    fn create_multiple(player_count: u8) -> Vec<Player> {
        let mut players = vec![];

        for _ in 0..player_count {
            players.push(Self::default());
        }

        players
    }

    /// Scores a players hand of cards.
    fn score_hand(&self) -> u32 {
        self.hand.iter().fold(0, |sum, card| sum + card.value())
    }
}

impl Default for Player {
    fn default() -> Self {
        Player { hand: Vec::new() }
    }
}

use cards::card_printer::display_hand;
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
    fn max_players(mut self, count: u8) -> GameBuilder {
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
            players: Player::create_multiple(self.player_count),
            turn: 0,
        }
    }
}

/// Holds game state.
struct Game {
    deck: Vec<Card>,
    players: Vec<Player>,
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
        self.players.iter().all(|card| card.hand.len() > 2)
    }

    /// Calculate all the players cards and prints the winner.
    fn show_winner(&self) {
        let mut winner = 0;
        let mut winning_score = 0;

        for (index, player) in self.players.iter().enumerate() {
            let score = player.score_hand();

            if score > winning_score {
                winning_score = score;
                winner = index
            }
        }
        println!("Player {} won!!!!", winner);
    }

    /// Shows the scores and hands of each player.
    fn show_results(&self) {
        for (index, player) in self.players.iter().enumerate() {
            let name = format!("Player {}", index);
            display_hand(&player.hand, &name);

            println!("Score: {}", player.score_hand());
        }
    }

    /// Advances the game forward one turn.
    fn advance(&mut self) {
        let card = self.deck.pop().unwrap();
        let player = self.turn % self.players.len();

        println!(
            "Turn {}: Player {} drew a {} with a value of {}.",
            self.turn,
            player,
            card.nomenclature(),
            card.value()
        );

        self.players[player].hand.push(card);
        self.players[player].hand.sort();

        self.turn += 1;
    }
}

fn main() {
    let mut game: Game = GameBuilder::new().max_players(3).spawn();

    game.shuffle_deck();

    while !game.is_over() {
        game.advance();
    }

    game.show_winner();

    game.show_results();
}
