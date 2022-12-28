

//use usb_ids;//::{self};//, FromId};



//use core::{time};//, slice::SlicePattern};

mod errors;
mod helpers;
mod reader;
mod commands;



use errors::UemGeneralError;
use reader::*;
//use errors::*; 



fn main() {
    let mut uem_readers = find_readers();

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

    if uem_reader.transceive(vec![0x05_u8, 0x01_u8]).is_err() {
        return;
    }

    if uem_reader.close().is_err() {
        return;
    };

}