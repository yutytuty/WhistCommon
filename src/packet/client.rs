use crate::logic::table::{Card, Suit};

#[derive(Debug, Eq, PartialEq)]
pub struct ClientPacket {
    pub card: Card,
}

impl ClientPacket {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(self.card.value.to_be_bytes().iter());
        bytes.extend(self.card.suit.to_be_bytes().iter());
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<ClientPacket> {
        if bytes.len() < 5 {
            return None;
        }
        let value = i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        match Suit::from_be_bytes([bytes[4]]) {
            Some(suit) => Some(ClientPacket {
                card: Card { value, suit },
            }),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_packet_to_bytes() {
        let test_packet = ClientPacket {
            card: Card {
                value: 8,
                suit: Suit::Hearts,
            },
        };
        let bytes = test_packet.to_bytes();
        let expected_bytes = [0, 0, 0, 8, 3];

        assert_eq!(bytes, expected_bytes);
    }

    #[test]
    fn test_client_packet_from_bytes() {
        let bytes = [0, 0, 0, 8, 3];
        let packet_from_bytes = ClientPacket::from_bytes(&bytes).unwrap();
        let expected_packet = ClientPacket {
            card: Card {
                value: 8,
                suit: Suit::Hearts,
            },
        };

        assert_eq!(packet_from_bytes, expected_packet);
    }
}
