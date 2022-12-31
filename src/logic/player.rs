use crate::logic::table::{Card, Deck, Table};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Player {
    pub name: String,
    hand: Vec<Card>,
}

impl Player {
    pub fn new(name: &str) -> Player {
        Player {
            name: name.to_string(),
            hand: vec![],
        }
    }

    pub fn draw(&mut self, deck: &mut Deck) -> Option<Card> {
        return if let Some(card) = deck.draw() {
            self.hand.push(card.clone());
            Some(card)
        } else {
            None
        };
    }

    pub fn play_card_from_hand(&mut self, table: &mut Table, index: usize) {
        table.place_card(self.clone(), self.hand.remove(index));
    }

    pub fn play_card(&self, table: &mut Table, card: Card) {
        table.place_card(self.clone(), card);
    }

    pub fn get_hand(&self) -> Vec<Card> {
        self.hand.clone()
    }
}

#[cfg(test)]
pub mod tests {
    use lazy_static::lazy_static;

    use crate::logic::table::Suit;

    use super::*;

    lazy_static! {
        pub static ref TEST_PLAYER_1: Player = Player {
            name: "Alice".to_string(),
            hand: vec![Card {
                value: 5,
                suit: Suit::Spades
            }],
        };
        pub static ref TEST_PLAYER_2: Player = Player {
            name: String::from("Bob"),
            hand: vec![Card {
                value: 5,
                suit: Suit::Hearts
            }],
        };
        pub static ref TEST_PLAYER_3: Player = Player {
            name: String::from("Charlie"),
            hand: vec![Card {
                value: 6,
                suit: Suit::Spades
            }],
        };
        pub static ref TEST_PLAYER_4: Player = Player {
            name: String::from("Dave"),
            hand: vec![Card {
                value: 4,
                suit: Suit::Spades
            }],
        };
    }
}
