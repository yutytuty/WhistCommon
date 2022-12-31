use crate::logic::table::{Card, Suit};

#[derive(Debug, Eq, PartialEq)]
pub struct ServerPacket {
    pub card: Card,
    num_seats_to_right: i32,
}

impl ServerPacket {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(self.card.value.to_be_bytes().iter());
        bytes.extend((self.card.suit as u8).to_be_bytes().iter());
        bytes.extend(self.num_seats_to_right.to_be_bytes().iter());
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 9 {
            return None;
        }
        let value = i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        match Suit::from_be_bytes([bytes[4]]) {
            Some(suit) => {
                let card = Card { value, suit };
                let num_seats_to_right = i32::from_be_bytes([bytes[5], bytes[6], bytes[7], bytes[8]]);
                Some(Self { card, num_seats_to_right })
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_packet_to_bytes() {
        let card = Card {
            value: 10,
            suit: Suit::Spades,
        };
        let num_seats_to_right = 3;
        let packet = ServerPacket { card, num_seats_to_right };

        let expected_bytes = vec![0, 0, 0, 10, 4, 0, 0, 0, 3];
        assert_eq!(packet.to_bytes(), expected_bytes);
    }

    #[test]
    fn test_server_packet_from_bytes() {
        let bytes = vec![0, 0, 0, 10, 4, 0, 0, 0, 3];
        let expected_packet = ServerPacket {
            card: Card {
                value: 10,
                suit: Suit::Spades,
            },
            num_seats_to_right: 3,
        };
        assert_eq!(ServerPacket::from_bytes(&bytes), Some(expected_packet));
    }
}
