use crate::reader::*;
use crate::errors::*;

//use std::sync::Weak;
use std::cell::RefCell;
use std::pin::Pin;
use std::rc::{Rc, Weak};

use rusb::UsbContext;

use std::borrow::Borrow;

pub struct Commands<T: UsbContext> {
    reader: Weak<RefCell<UemReader<T>>>,
}

impl<T: UsbContext> Commands<T> {
    pub fn beep(self) -> core::result::Result<(), UemGeneralError> {
        let raw_reader: RefCell<UemReader<T>> = unsafe {*self.reader.into_raw()};
        raw_reader.borrow_mut().transceive(vec![0x05_u8, 0x01_u8]).map(|_| ())
    }
}

