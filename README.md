# Library for RFID readers by MicroEM company

The library allows control of RFID readers produced by [MicroEM company](https://microem.ru) by implementing a protocol used in these readers.

The crate comes with basic objects and a `send` method used to interact with a reader. The byte sequences used in this method should conform with official documentation for the reader which can be found on [MicroEM website.](https://microem.ru)

Currently the crate has been tested with LibUsb on Debian 11.5 x64 and Windows 7x64.

Note that in order to work with Windows you need to [install libusb driver first](https://github.com/libusb/libusb/wiki/Windows#how-to-use-libusb-on-windows).

On Linux you need to add write permissions to the device you want to use, e.g.:
```
sudo chmod o+w /dev/bus/usb/002/008
```

## Usage

```rust
    use uem_reader::{
        reader::{
            UemReaderInternal,
            usb::find_usb_readers
        },
        commands::{
            UemCommandsTrait,
            reader::UemCommandsReaderTrait, 
            cards::{
                UemCommandsCardsTrait,
                UemActivateParameters,
                mifare::{
                    UemCommandsCardsMifareTrait,
                    classic::UemCommandsCardsMifareClassicTrait
                },
            },
        },
    };

    // Search system for USB readers
    let mut uem_readers = find_usb_readers();

    // Quit if no readers found
    if uem_readers.is_empty() {
        return;
    }

    // Pick the first reader in the vector
    let uem_reader = uem_readers.get_mut(0);

    // Check if the vector returned an option with valid reader object
    if uem_reader.is_none() {
        return;
    }

    // Unwrap the option
    let uem_reader = uem_reader.unwrap();
    
    // Open USB interface connection
    if uem_reader.open().is_err() {
        return;
    }

    // Beep 1 time using command byte vector
    if uem_reader.send(&vec![0x05_u8, 0x01_u8]).is_err() {
        return;
    }

    // Beep 5 times using command grouping objects as separate variables
    let mut uem_cmds = uem_reader.commands();
    let mut uem_cmds_reader = uem_cmds.reader();
    if uem_cmds_reader.beep(5).is_err() {
        return;
    }

    // Beep 3 times using command grouping objects inline
    if uem_reader.commands().reader().beep(3).is_err() {
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
```

## License

This work is dual-licensed under MIT or Apache 2.0.
You can choose between one of them if you use this work.

`SPDX-License-Identifier: MIT OR Apache-2.0`
