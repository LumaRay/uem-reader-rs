//! Crate card related structures
//! 

#![allow(dead_code)]

use enum_iterator::Sequence;

// #[repr(u8)]
// #[derive(Debug, Default, PartialEq, Sequence, Clone)]
// pub enum UemCardStandard {
//     #[default]
//     Iso14443a = 0x00,
//     Iso14443b = 0x01,
//     Iso15693 = 0x02,
// }

#[repr(u8)]
#[derive(Debug, Default, PartialEq, Sequence, Clone, Copy)]
pub enum UemCardBaudrates {
    #[default]
    Baud106kbps = 0b00,
    Baud212kbps = 0b01,
    Baud424kbps = 0b10,
    Baud848kbps = 0b11,
}

#[derive(Debug, Clone)]
/// ISO14443A card type object
pub struct UemCardIso14443A {
    /// Answer to request - 2 bytes
    pub atq: Vec<u8>,
    /// Select Acknowledge byte 
    pub sak: u8,
    /// Unique identifier - 4/7/10 bytes
    pub uid: Vec<u8>,
    /// Answer to select - returned after switching
    /// to T=CL mode
    pub ats: Vec<u8>,
}

#[derive(Debug, Clone)]
/// ISO14443B card type object
pub struct UemCardIso14443B {
    /// Buffer length identifier
    pub mbli: u8,
    /// Unique identifier - 4 bytes
    pub pupi: Vec<u8>,
    /// Application data - 4 bytes
    pub app_data: Vec<u8>,
    /// Protocol information - 3 bytes
    pub prot_info: Vec<u8>,
    /// Answer to request
    pub atq: Vec<u8>,
}

/// General placeholder for a card object
pub enum UemCard {
    Iso14443A(UemCardIso14443A),
    Iso14443B(UemCardIso14443B),
}

impl UemCard {
    /// Returns unique identifier of a card
    pub fn uid(&self) -> &Vec<u8> {
        match self {
            Self::Iso14443A(card) => &card.uid,
            Self::Iso14443B(card) => &card.pupi,
        }
    }
}