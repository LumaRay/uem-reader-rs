// test sequentially using cargo test -- --test-threads 1

mod errors;
mod helpers;
mod card;
mod reader;
mod commands;

use reader::*;
use commands::*;
use commands::{
    reader::*, 
    cards::*,
    cards::mifare::*,
    cards::mifare::classic::*
};

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
    if uem_cmds_reader.beep(5).is_err() {
        uem_reader.close();
        return;
    }

    // if (uem_reader.commands.into() as UemReader<GlobalContext>).beep().is_err() {
    //if (uem_reader.commands.as_mut()).beep(3).is_err() {
    if uem_reader.commands().reader().beep(3).is_err() {
        uem_reader.close();
        return;
    }

    let card = uem_reader.commands().cards().activate_a(&UemActivateParameters{
        // switch_to_tcl: true,
        ..Default::default()
    });

    if card.is_err() {
        uem_reader.close();
        return;
    }

    let card = card.unwrap();

    let res = uem_reader.commands().cards().mifare().classic()
        .authenticate_key_a(
            &card, 
            &[0xFF; 6], 
            1
        );

    if res.is_err() {
        uem_reader.close();
        return;
    }

    let res = uem_reader.commands().cards().mifare().classic()
        .read(1, 1);

    if res.is_err() {
        uem_reader.close();
        return;
    }

    let prev_data = res.unwrap();
    let mut new_data = prev_data.clone();
    new_data[0] = 0xFF;

    let res = uem_reader.commands().cards().mifare().classic()
        .write(new_data.clone(), 1, 1);

    if res.is_err() {
        uem_reader.close();
        return;
    }

    let res = uem_reader.commands().cards().mifare().classic()
        .read(1, 1);

    if res.is_err() {
        uem_reader.close();
        return;
    }

    if res.unwrap() != new_data {
        println!("error!");
        uem_reader.close();
        return;
    }

    let res = uem_reader.commands().cards().mifare().classic()
        .write(prev_data.clone(), 1, 1);

    if res.is_err() {
        uem_reader.close();
        return;
    }

    let res = uem_reader.commands().cards().mifare().classic()
        .read(1, 1);

    if res.is_err() {
        uem_reader.close();
        return;
    }

    if res.unwrap() != prev_data {
        println!("error!");
        uem_reader.close();
        return;
    }

    if uem_reader.close().is_err() {
        return;
    };

}