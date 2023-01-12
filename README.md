# Library for RFID readers by MicroEM company

The library allows control of RFID readers produced by [MicroEM company](https://microem.ru) by implementing a protocol used in these readers.

The crate comes with basic objects and a `send` method used to interact with a reader. The byte sequences used in this method should conform with official documentation for the reader which can be found on [MicroEM website.](https://microem.ru)

Currently the crate has been tested with LibUsb on Debian 11.5 x64 and Windows 7x64.

Note that in order to work with Windows you need to [install libusb driver first](https://github.com/libusb/libusb/wiki/Windows#how-to-use-libusb-on-windows).

## Usage

```rust
    use uem_reader::reader::*;
    use uem_reader::commands::*;
    use uem_reader::commands::reader::*;
    use uem_reader::reader::usb::find_usb_readers;

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
    if uem_reader.send(vec![0x05_u8, 0x01_u8]).is_err() {
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

    // Close USB interface connection
    if uem_reader.close().is_err() {
        return;
    };
```
