//! Grouping of commands related to Mifare cards family

pub mod classic;

use crate::reader::*;
use crate::commands::cards::mifare::classic::*;

/// Structure for commands to interact
/// with Mifare cards
pub struct UemCommandsCardsMifare<'a> {
    reader: &'a UemReader,
}

/// Accessing Mifare cards related commands group
pub trait UemCommandsCardsMifareTrait {
    fn mifare(&mut self) -> UemCommandsCardsMifare;
}

impl<'a> UemCommandsCardsMifareClassicTrait for UemCommandsCardsMifare<'a> {   
    fn classic(&mut self) -> UemCommandsCardsMifareClassic {
        UemCommandsCardsMifareClassic::new(self.as_reader())
    }
}

impl<'a> UemCommandsCardsMifare<'a> {
    pub(crate) fn new(rd: &'a UemReader) -> Self {
        UemCommandsCardsMifare {reader: rd}
    }

    pub(crate) fn as_reader(&self) -> &'a UemReader {
        self.reader
    }
}

