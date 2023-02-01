//! Commands relative to control behavior of
//! a reader itself

#![allow(dead_code)]

use crate::reader::*;
use crate::errors::*;

use enum_iterator::Sequence;

#[repr(u8)]
#[derive(Debug, PartialEq, Sequence, Clone, Copy)]
/// LED color combinations
pub enum UemColor {
    /// LED is off
    Off = 0b000,
    Red = 0b001,
    Green = 0b010,
    Blue = 0b100,
    Yellow = 0b011,
    Magenta = 0b101,
    Cyan = 0b110,
    White = 0b111,
}

/// Structure for commands controlling 
/// a reader itself
pub struct UemCommandsReader<'a> {
    reader: &'a UemReader,
}

/// Accessing reader related commands group
pub trait UemCommandsReaderTrait {
    fn reader(&mut self) -> UemCommandsReader;
}

impl<'a> UemCommandsReader<'a> {
    pub(crate) fn new(rd: &'a UemReader) -> Self {
        UemCommandsReader {reader: rd}
    }

    /// Make short sound signals of specific count
    /// 
    /// # Arguments
    ///
    /// * `count` - Number of beeps to perform
    /// 
    /// # Returns
    /// 
    /// `Ok(())` on success, otherwise returns an error.
    /// 
    /// # Example
    /// 
    /// ```
    /// # use uem_reader::reader::{UemReaderInternalTrait, usb::find_usb_readers};
    /// # use uem_reader::commands::{UemCommandsTrait, reader::*};
    /// # let mut uem_readers = find_usb_readers();
    /// # if uem_readers.is_empty() { return; }
    /// # let uem_reader = uem_readers.get_mut(0);
    /// # if uem_reader.is_none() { return; }
    /// # let uem_reader = uem_reader.unwrap();
    /// # if uem_reader.open().is_err() { return; }
    /// // Beep 5 times
    /// if uem_reader.commands().reader()
    ///     .beep(5)
    /// .is_err() {
    ///     return;
    /// }
    /// # if uem_reader.close().is_err() { return; }
    /// ```
    pub fn beep(&mut self, count: u8) -> UemResult {
        if count < 1 {
            return Err(UemError::IncorrectParameter);
        }
        let mut raw_reader = self.reader.lock().unwrap();
        raw_reader.send(&vec![0x05, count]).map(|_| ())
    }

    /// Blink `count` times with led of specific color
    /// and remain with other color switched on
    /// 
    /// # Arguments
    ///
    /// * `count` - Number of blinks to perform
    /// * `color` - [color](UemColor) to blink with
    /// * `post_color` - [color](UemColor) to leave on
    /// 
    /// # Returns
    /// 
    /// `Ok(())` on success, otherwise returns an error.
    /// 
    /// # Example
    /// 
    /// ```
    /// # use uem_reader::reader::{
    ///     UemReaderInternalTrait,
    ///     usb::find_usb_readers};
    /// # use uem_reader::commands::{UemCommandsTrait, reader::*};
    /// # let mut uem_readers = find_usb_readers();
    /// # if uem_readers.is_empty() { return; }
    /// # let uem_reader = uem_readers.get_mut(0);
    /// # if uem_reader.is_none() { return; }
    /// # let uem_reader = uem_reader.unwrap();
    /// # if uem_reader.open().is_err() { return; }
    /// // Blink 3 times with green and remain yellow
    /// if uem_reader.commands().reader()
    ///     .led(3, UemColor::Green, UemColor::Yellow)
    /// .is_err() {
    ///     return;
    /// }
    /// # if uem_reader.close().is_err() { return; }
    /// ```
    pub fn led(&mut self, count: u8, color: UemColor, post_color: UemColor) -> UemResult {
        let mut raw_reader = self.reader.lock().unwrap();
        raw_reader.send(&vec![0x07, color as u8, count, post_color as u8]).map(|_| ())
    }

    /// Turn radio chip on
    /// 
    /// # Arguments
    /// 
    /// - `on` - `true` if needed to power on the radio, 
    /// otherwise `false`
    /// 
    /// # Returns
    /// 
    /// `Ok(())` on success, otherwise returns an [`UemError`](UemError).
    /// 
    /// # Example
    /// 
    /// ```
    /// # use uem_reader::reader::{UemReaderInternalTrait, usb::find_usb_readers};
    /// # use uem_reader::commands::{UemCommandsTrait, reader::*};
    /// # let mut uem_readers = find_usb_readers();
    /// # if uem_readers.is_empty() { return; }
    /// # let uem_reader = uem_readers.get_mut(0);
    /// # if uem_reader.is_none() { return; }
    /// # let uem_reader = uem_reader.unwrap();
    /// # if uem_reader.open().is_err() { return; }
    /// let mut uem_cmds = uem_reader.commands();
    /// let mut uem_cmds_reader = uem_cmds.reader();
    /// if uem_cmds_reader.power_radio(true).is_err() {
    ///     return;
    /// }
    /// # if uem_reader.close().is_err() { return; }
    /// ```
    pub fn power_radio(&mut self, on: bool) -> UemResult {
        let mut raw_reader = self.reader.lock().unwrap();
        match on {
            true => raw_reader.send(&vec![0x10]).map(|_| ()),
            false => raw_reader.send(&vec![0x04, 0x80, 0x01]).map(|_| ())
        }
    }

    /// Switch radio field off for a specified duration
    /// 
    /// By switching off radio field, cards in the field
    /// are beind unpowered and thus reset.
    /// 
    /// # Arguments
    ///
    /// * `duration` - Number of milliseconds
    /// to switch off radio field. If set to 0,
    /// the field will be switched off permanently.
    /// The field can be switched on again with
    /// the same command and non-zero duration.
    /// 
    /// # Returns
    /// 
    /// `Ok(())` on success, otherwise returns an error.
    /// 
    /// # Example
    /// 
    /// ```
    /// # use uem_reader::reader::{UemReaderInternalTrait, usb::find_usb_readers};
    /// # use uem_reader::commands::{UemCommandsTrait, reader::*};
    /// # let mut uem_readers = find_usb_readers();
    /// # if uem_readers.is_empty() { return; }
    /// # let uem_reader = uem_readers.get_mut(0);
    /// # if uem_reader.is_none() { return; }
    /// # let uem_reader = uem_reader.unwrap();
    /// # if uem_reader.open().is_err() { return; }
    /// // Switch off radio for 10 ms
    /// if uem_reader.commands().reader()
    ///     .radio_off_on(10)
    /// .is_err() {
    ///     return;
    /// }
    /// # if uem_reader.close().is_err() { return; }
    /// ```
    pub fn radio_off_on(&mut self, duration: u16) -> UemResult {
        let mut raw_reader = self.reader.lock().unwrap();
        raw_reader.send(&vec![0x05, 
            (duration & 0x00FF) as u8,
            ((duration & 0xFF00) >> 8) as u8]
        ).map(|_| ())
    }

    /// Read reader version
    /// 
    /// Reader version is a 6 bytes vector
    /// 
    /// # Returns
    /// 
    /// `Ok(Vec<u8>)` containing the version,
    /// otherwise [`UemError`](UemError).
    /// 
    /// # Example
    /// 
    /// ```
    /// # use uem_reader::reader::{UemReaderInternalTrait, usb::find_usb_readers};
    /// # use uem_reader::commands::{UemCommandsTrait, reader::*};
    /// # let mut uem_readers = find_usb_readers();
    /// # if uem_readers.is_empty() { return; }
    /// # let uem_reader = uem_readers.get_mut(0);
    /// # if uem_reader.is_none() { return; }
    /// # let uem_reader = uem_reader.unwrap();
    /// # if uem_reader.open().is_err() { return; }
    /// match uem_reader.commands().reader()
    /// .get_version() {
    ///     Ok(ver) => assert_eq!(ver.len(), 6),
    ///     Err(err) => {
    ///         uem_reader.close();
    ///         println!("{:?}", err);
    ///         assert!(false);
    ///     }
    /// }
    /// # if uem_reader.close().is_err() { return; }
    /// ```
    pub fn get_version(&mut self) -> UemResultVec {
        let mut raw_reader = self.reader.lock().unwrap();
        raw_reader.send(&vec![0x64])
    }

    /// Read reader serial
    /// 
    /// Reader serial is a 4 bytes vector
    /// 
    /// # Returns
    /// 
    /// `Ok(Vec<u8>)` containing the serial,
    /// otherwise [`UemError`](UemError).
    /// 
    /// # Example
    /// 
    /// ```
    /// # use uem_reader::reader::{UemReaderInternalTrait, usb::find_usb_readers};
    /// # use uem_reader::commands::{UemCommandsTrait, reader::*};
    /// # let mut uem_readers = find_usb_readers();
    /// # if uem_readers.is_empty() { return; }
    /// # let uem_reader = uem_readers.get_mut(0);
    /// # if uem_reader.is_none() { return; }
    /// # let uem_reader = uem_reader.unwrap();
    /// # if uem_reader.open().is_err() { return; }
    /// match uem_reader.commands().reader()
    /// .get_serial() {
    ///     Ok(ser) => assert_eq!(ser.len(), 4),
    ///     Err(err) => {
    ///         uem_reader.close();
    ///         println!("{:?}", err);
    ///         assert!(false);
    ///     }
    /// }
    /// # if uem_reader.close().is_err() { return; }
    /// ```
    pub fn get_serial(&mut self) -> UemResultVec {
        let mut raw_reader = self.reader.lock().unwrap();
        raw_reader.send(&vec![0x22])
    }
}
