//! Crate commands object type

pub mod reader;
pub mod cards;

use crate::reader::*;
use crate::commands::{reader::*, cards::*};

/// Structure for grouping commands in general
pub struct UemCommands<'a> {
    reader: &'a UemReader,
}

/// Accessing general commands group
pub trait UemCommandsTrait {
    fn commands(&mut self) -> UemCommands;
}

impl<'a> UemCommandsReaderTrait for UemCommands<'a> {  
    fn reader(&mut self) -> UemCommandsReader {
        UemCommandsReader::new(self.as_reader())
    }
}

impl<'a> UemCommandsCardsTrait for UemCommands<'a> {  
    fn cards(&mut self) -> UemCommandsCards {
        UemCommandsCards::new(self.as_reader())
    }
}

impl<'a> UemCommands<'a> {
    pub(crate) fn new(rd: &'a UemReader) -> Self {
        UemCommands {reader: rd}
    }
    
    pub(crate) fn as_reader(&self) -> &'a UemReader {
        self.reader
    }
}