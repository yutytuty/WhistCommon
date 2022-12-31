use bytes::{Buf, BytesMut};

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

#[derive(Debug, Eq, PartialEq)]
pub struct ClientHandshakePacket {
    pub num_names: u8,
    pub names: Vec<String>,
}

impl ClientHandshakePacket {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.push(self.num_names);

        for name in &self.names {
            let name_bytes = name.as_bytes();
            bytes.push(name_bytes.len() as u8);
            bytes.extend_from_slice(name_bytes);
        }

        bytes
    }

    pub fn from_bytes(raw: &[u8]) -> Option<ClientHandshakePacket> {
        if raw.len() < 1 {
            return None;
        }

        let mut bytes = BytesMut::from(raw);
        let num_names = bytes.get_u8();

        let mut names = Vec::new();

        for _ in 0..num_names {
            let name_length = bytes.get_u8();
            let mut name_buffer = vec![0; name_length as usize];
            bytes.copy_to_slice(&mut name_buffer);
            println!("{}", bytes.remaining());
            let name = String::from_utf8_lossy(&name_buffer).to_string();
            names.push(name);
        }

        Some(ClientHandshakePacket { num_names, names })
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

    #[test]
    fn test_client_handshake_packet_to_bytes() {
        let packet = ClientHandshakePacket {
            num_names: 2,
            names: vec![String::from("Alice"), String::from("Bob")],
        };

        let expected: Vec<u8> = vec![
            0x02, 0x05, 0x41, 0x6C, 0x69, 0x63, 0x65, 0x03, 0x42, 0x6F, 0x62,
        ];

        let result = packet.to_bytes();

        assert_eq!(expected, result);
    }

    #[test]
    fn test_client_handshake_packet_from_bytes() {
        let packet = ClientHandshakePacket {
            num_names: 2,
            names: vec![String::from("Alice"), String::from("Bob")],
        };

        let raw: Vec<u8> = vec![
            0x02, 0x05, 0x41, 0x6C, 0x69, 0x63, 0x65, 0x03, 0x42, 0x6F, 0x62,
        ];

        let result = ClientHandshakePacket::from_bytes(&raw).unwrap();

        assert_eq!(packet, result);
    }
}
