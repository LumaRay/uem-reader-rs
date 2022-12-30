use crate::reader::*;
use crate::errors::*;

//use std::sync::Weak;
use std::cell::RefCell;
use std::pin::Pin;
use std::rc::{Rc, Weak};

use rusb::UsbContext;

use std::borrow::Borrow;

pub struct Commands<T: UsbContext> {
    // reader: Weak<RefCell<UemReader<T>>>,
    pub reader: *mut UemReader<T>,
}

impl<T: UsbContext> Default for Commands<T> {
    fn default() -> Self {
        Self { reader: std::ptr::null_mut() }
    }
}

impl<T: UsbContext> Commands<T> {
    /// Make short beep sound
    ///
    /// # Examples
    ///
    /// //```rust
    /// //assert_eq!(min( 0,   14),    0);
    /// //assert_eq!(min( 0, -127), -127);
    /// //assert_eq!(min(42,  666),   42);
    /// //```
    pub fn beep(self, count: i32) -> core::result::Result<(), UemError> {
        if count < 1 || count > 255 {
            return Err(UemError::IncorrectParameter);
        }
        // let raw_reader: RefCell<UemReader<T>> = unsafe {*self.reader.into_raw()};
        // raw_reader.borrow_mut().transceive(vec![0x05_u8, 0x01_u8]).map(|_| ())
        // let raw_reader = unsafe {*self.reader};
        let raw_reader = unsafe{ &mut *self.reader };
        raw_reader.transceive(vec![0x05_u8, count as u8]).map(|_| ())
    }
}

