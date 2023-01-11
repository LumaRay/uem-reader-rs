//! COM port reader interface (RS232/485)
//! Not implemented!

#![allow(dead_code)]

use std::{time::Duration};

use crate::reader::*;
use crate::reader::processing::*;

#[derive(Debug, Default)]
struct ReaderRs {
    timeout: Duration,
    port: u8,
    ncommand: u8,
}

impl CommandsCounter for ReaderRs {
    fn commands_count(&self) -> u8 {
        self.ncommand
    }

    fn increment_commands(&mut self) {
        if self.commands_count() == u8::MAX {
            self.ncommand = 0;
        }
        self.ncommand += 1;
    }
}

impl ReaderRs {
    /// Open COM interface
    //#![warn(missing_docs)]
    pub fn open(&mut self) -> UemResult {
        Ok(())
    }        

    /// close opened COM interface
    pub fn close(&mut self) -> core::result::Result<(), UemError> {
        Ok(())
    }

    #[allow(unused_variables)]
    /// Send command to a reader and receive response
    pub fn send(&mut self, command: Vec<u8>) -> UemResultVec {
        Ok(Vec::new())
    }
}

/// Searches system for MicroEM readers
/// on COM ports
pub fn find_rs_readers() -> Vec<UemReader> {
    Vec::new()
}