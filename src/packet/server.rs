use std::io::{Read, Result};

use bytes::{Buf, BytesMut};

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
                let num_seats_to_right =
                    i32::from_be_bytes([bytes[5], bytes[6], bytes[7], bytes[8]]);
                Some(Self {
                    card,
                    num_seats_to_right,
                })
            }
            None => None,
        }
    }
}

pub struct ServerHandshakePacket {
    pub num_names: u8,
    pub names: Vec<String>,
}

impl ServerHandshakePacket {
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

    pub fn from_bytes(raw: &[u8]) -> Option<ServerHandshakePacket> {
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

        Some(ServerHandshakePacket { num_names, names })
    }

    pub fn get_client_handshake_packet<R: Read>(reader: &mut R) -> Result<ServerHandshakePacket> {
        let mut num_names_buf = [0];
        reader.read_exact(&mut num_names_buf)?;
        let num_names = num_names_buf[0];

        let mut names = Vec::new();

        for _ in 0..num_names {
            let mut name_length_buf = [0];
            reader.read_exact(&mut name_length_buf)?;
            let name_length = name_length_buf[0] as usize;

            let mut name_buf = vec![0; name_length];
            reader.read_exact(&mut name_buf)?;

            let name = String::from_utf8_lossy(&name_buf).to_string();
            names.push(name);
        }

        Ok(ServerHandshakePacket { num_names, names })
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
        let packet = ServerPacket {
            card,
            num_seats_to_right,
        };

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
