

//use usb_ids;//::{self};//, FromId};

use rusb::{UsbContext, GlobalContext};

//use core::{time};//, slice::SlicePattern};

mod errors;
mod helpers;
mod reader;
mod reader_usb;
mod commands;
mod commands_reader;



use errors::UemError;
use reader::*;
//use reader_usb::*;
//use errors::*; 
use commands::*;
use commands_reader::*;

// https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html
// cargo doc --no-deps --open

fn main() {
//! This is my first rust crate
    let mut uem_readers = find_usb_readers();

    if uem_readers.is_empty() {
        return;
    }

    let uem_reader = uem_readers.get_mut(0);

    if uem_reader.is_none() {
        return;
    }

    let uem_reader = uem_reader.unwrap();
    
    if uem_reader.open().is_err() {
        return;
    }

    // if uem_reader.transceive(vec![0x05_u8, 0x01_u8]).is_err() {
    //     return;
    // }
    let mut uem_cmds = uem_reader.commands();
    let mut uem_cmds_reader = uem_cmds.reader();
    uem_cmds_reader.beep(5);
    // if (uem_reader.commands.into() as UemReader<GlobalContext>).beep().is_err() {
    //if (uem_reader.commands.as_mut()).beep(3).is_err() {
    if uem_reader.commands().reader().beep(3).is_err() {
        return;
    }

    if uem_reader.close().is_err() {
        return;
    };

}