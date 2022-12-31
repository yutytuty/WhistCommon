use std::collections::HashMap;

use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::logic::player::Player;

#[derive(EnumIter, Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Suit {
    Spades = 4,
    Hearts = 3,
    Diamonds = 2,
    Clubs = 1,
}

impl Suit {
    pub fn to_string(&self) -> String {
        match *self {
            Suit::Spades => "Spades",
            Suit::Hearts => "Hearts",
            Suit::Diamonds => "Diamonds",
            Suit::Clubs => "Clubs",
        }
        .to_string()
    }

    pub fn to_be_bytes(&self) -> [u8; 1] {
        (*self as u8).to_be_bytes()
    }

    pub fn from_be_bytes(bytes: [u8; 1]) -> Option<Self> {
        match u8::from_be_bytes(bytes) {
            4 => Some(Suit::Spades),
            3 => Some(Suit::Hearts),
            2 => Some(Suit::Diamonds),
            1 => Some(Suit::Clubs),
            _ => None,
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Card {
    pub value: i32,
    pub suit: Suit,
}

impl Card {
    pub fn is_greater(&self, other: &Card) -> bool {
        if self.value == other.value {
            return self.suit as u8 > other.suit as u8;
        }
        self.value > other.value
    }

    pub fn to_string(&self) -> String {
        String::from(format!("{} of {}", self.value, self.suit.to_string()))
    }
}

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut deck = Deck { cards: vec![] };
        deck.initialize_deck();
        deck
    }

    pub fn initialize_deck(&mut self) {
        for suit in Suit::iter() {
            for value in 2..14 {
                self.cards.push(Card { value, suit })
            }
        }
        self.cards.shuffle(&mut thread_rng());
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

pub struct Table {
    table: HashMap<Player, Option<Card>>,
}

impl Table {
    pub fn new(players: [Player; 4]) -> Table {
        let mut hashmap = HashMap::new();
        for player in players {
            hashmap.insert(player, None);
        }

        Table { table: hashmap }
    }

    pub fn place_card(&mut self, player: Player, card: Card) {
        if self.table.contains_key(&player) {
            self.table.insert(player, Some(card));
        }
    }

    pub fn strongest_card_holder(&self) -> Option<Player> {
        let mut strongest_player = None;
        let mut strongest_card = None;
        for (player, card) in self.table.iter() {
            if let (Some(strongest_card_to_test), Some(current_card)) = (strongest_card, card) {
                if current_card.is_greater(strongest_card_to_test) {
                    strongest_card = Some(current_card);
                    strongest_player = Some(player.clone());
                }
            } else {
                strongest_card = Option::from(card);
                strongest_player = Some(player.clone());
            }
        }
        strongest_player
    }
}

#[cfg(test)]
mod tests {
    use crate::logic::player::tests::*;

    use super::*;

    #[test]
    fn test_is_greater() {
        let card_1 = Card {
            value: 5,
            suit: Suit::Spades,
        };
        let card_2 = Card {
            value: 5,
            suit: Suit::Hearts,
        };
        let card_3 = Card {
            value: 6,
            suit: Suit::Spades,
        };
        let card_4 = Card {
            value: 4,
            suit: Suit::Spades,
        };
        let card_5 = Card {
            value: 5,
            suit: Suit::Clubs,
        };

        assert!(card_1.is_greater(&card_2));
        assert!(card_3.is_greater(&card_1));
        assert!(!card_1.is_greater(&card_3));
        assert!(card_1.is_greater(&card_4));
        assert!(card_2.is_greater(&card_5));
    }

    #[test]
    fn test_strongest_card_holder() {
        let players = [
            TEST_PLAYER_1.clone(),
            TEST_PLAYER_2.clone(),
            TEST_PLAYER_3.clone(),
            TEST_PLAYER_4.clone(),
        ];
        let mut table = Table::new([
            TEST_PLAYER_1.clone(),
            TEST_PLAYER_2.clone(),
            TEST_PLAYER_3.clone(),
            TEST_PLAYER_4.clone(),
        ]);

        for mut player in players {
            player.play_card_from_hand(&mut table, 0);
        }

        assert_eq!(table.strongest_card_holder(), Some(TEST_PLAYER_3.clone()));
    }
}
