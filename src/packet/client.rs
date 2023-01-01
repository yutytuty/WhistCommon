use std::io::{Read, Result};

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
    name: String,
}

impl ClientHandshakePacket {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        let name = &self.name;
        let name_bytes = name.as_bytes();
        bytes.push(name_bytes.len() as u8);
        bytes.extend_from_slice(name_bytes);
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<String> {
        if bytes.len() < 1 {
            return None;
        }

        let name_length = bytes[0] as usize;
        if bytes.len() < name_length + 1 {
            return None;
        }

        let name = String::from_utf8_lossy(&bytes[1..name_length + 1].to_vec()).to_string();

        Some(name)
    }

    pub fn get_client_handshake_packet<R: Read>(reader: &mut R) -> Result<ClientHandshakePacket> {
        let mut name_length_buf = [0];
        reader.read_exact(&mut name_length_buf)?;
        let name_length = name_length_buf[0];

        let mut name_buf = vec![0; name_length.into()];
        reader.read_exact(&mut name_buf)?;
        let name = String::from_utf8_lossy(&name_buf).to_string();
        Ok(ClientHandshakePacket { name })
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
            name: String::from("Alice"),
        };

        let expected: Vec<u8> = vec![0x05, 0x41, 0x6C, 0x69, 0x63, 0x65];

        let result = packet.to_bytes();

        assert_eq!(expected, result);
    }

    #[test]
    fn test_client_handshake_packet_from_bytes() {
        let expected = ClientHandshakePacket {
            name: String::from("Alice"),
        };

        let raw: Vec<u8> = vec![0x05, 0x41, 0x6C, 0x69, 0x63, 0x65];

        let result = ClientHandshakePacket::from_bytes(&raw).unwrap();

        assert_eq!(expected, ClientHandshakePacket { name: result });
    }

    #[test]
    fn test_get_client_handshake_packet() {
        let data: Vec<u8> = vec![0x05, 0x41, 0x6C, 0x69, 0x63, 0x65];
        let expected = ClientHandshakePacket {
            name: String::from("Alice"),
        };

        let mut reader = &data[..];
        let result = ClientHandshakePacket::get_client_handshake_packet(&mut reader).unwrap();

        assert_eq!(expected, result);
    }
}
