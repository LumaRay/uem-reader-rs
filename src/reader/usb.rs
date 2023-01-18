//! USB reader implementation

//use core::slice::SlicePattern;
use std::sync::{Arc, Mutex};
use std::{time::Duration};
use rand::Rng;
use rusb::{
    DeviceHandle, DeviceList, Language, 
    Device, UsbContext, Direction,
};

use crate::reader::*;
use crate::reader::processing::*;
use crate::errors::*;

const UEM_VID: u16 = 0xC251;
const UEM_PID: u16 = 0x130A;

#[derive(Debug, Default)]
struct ReaderUsb<T: UsbContext> {
    handle: Option<DeviceHandle<T>>,
    device: Option<Device<T>>,
    language: Option<Language>,
    timeout: Duration,
    ep_in_addr: u8,
    ep_out_addr: u8,
    ncommand: u8,
}

impl<T: UsbContext> CommandsCounter for ReaderUsb<T> {
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

impl<T: UsbContext> UemReaderInternal for ReaderUsb<T> {
    //#![warn(missing_docs)]
    /// Open USB interface
    fn open(&mut self) -> UemResult {
        if self.handle.is_some() {
            return Err(UemError::ReaderAlreadyConnected);
        }
        if let Ok(h) = self.device.take().unwrap().open() {
            if let Ok(l) = h.read_languages(TIMEOUT) {
                if !l.is_empty() {
                    self.language = Some(l[0]);
                }
            }
            self.handle = Some(h);
            self.timeout = TIMEOUT;
            return Ok(())
        }
        Err(UemError::ReaderConnectionFailed)
    }        

    /// Close opened USB interface
    fn close(&mut self) -> core::result::Result<(), UemError> {
        if self.handle.is_none() {
            return Err(UemError::ReaderNotConnected);
        }
        if let Some(h) = self.handle.take() {
            self.device = Some(h.device());
            return Ok(())
        }
        Ok(())
    }

    /// Send command directly to a USB reader
    fn send(&mut self, command: &[u8]) -> UemResultVec {
        
        if self.handle.is_none() {
            return Err(UemError::ReaderNotConnected);
        }
        if command.is_empty() {
            return Err(UemError::IncorrectParameter);
        }

        let send_buffer = prepare_command(self, &command);
        if send_buffer.is_empty() {
            return Err(UemError::IncorrectParameter);
        }

        let handle = self.handle.as_mut().unwrap();

        handle.claim_interface(0).map_err(|_| UemError::Access)?;

        let mut res = handle.write_bulk(self.ep_out_addr, send_buffer.as_slice(), TIMEOUT);

        if res.is_err() {
            return Err(UemError::NotTransacted);
        }

        let mut receive_buffer = vec![0u8; 256];

        res = handle.read_bulk(self.ep_in_addr, &mut receive_buffer, TIMEOUT);

        handle.release_interface(0).map_err(|_| UemError::Access)?;

        if res.is_err() {
            return Err(UemError::ReaderResponseFailure);
        }

        let response_length = res.unwrap();

        if response_length <= 6 {
            return Err(UemError::ReaderResponseFailure);
        }

        let response = parse_response(&receive_buffer[..response_length].to_vec())?;

        if (response.len() < 2) || (response[0] != command[0]) {
            return Err(UemError::ReaderIncorrectResponse);
        }

        if response[1] != 0x00 {
            if response.len() == 2 {
                return Err(UemError::ReaderUnsuccessful(UemInternalError::from_byte(response[1]), None));
            }
            return Err(UemError::ReaderUnsuccessful(UemInternalError::from_byte(response[1]), Some(response[2..].to_vec())));
        }

        Ok(response[2..].to_vec())
    }
}

/// Search system for MicroEM readers on USB ports
/// 
/// # Example
/// 
/// ```no_run
/// # use uem_reader::reader::*;
/// # use uem_reader::reader::usb::find_usb_readers;
/// 
/// // Search system for USB readers
/// let mut uem_readers = find_usb_readers();
/// 
/// // Quit if no readers found
/// if uem_readers.is_empty() {
///     return;
/// }
/// 
/// // Pick the first reader in the vector
/// let uem_reader = uem_readers.get_mut(0);
/// 
/// // Check if the vector returned an option with valid reader object
/// if uem_reader.is_none() {
///     return;
/// }
/// 
/// // Unwrap the option
/// let uem_reader = uem_reader.unwrap();
/// ```
pub fn find_usb_readers() -> Vec<UemReader> {
    let mut usb_readers: Vec<UemReader> = Vec::new();
    let devices = DeviceList::new();
    if let Err(_) = devices {
        return usb_readers;
    }
    for device in devices.unwrap().iter() {
        let device_desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue,
        };

        if  device_desc.vendor_id() != UEM_VID || 
            device_desc.product_id() != UEM_PID {
            continue
        }

        let mut usb_reader = ReaderUsb {
            ncommand: rand::thread_rng().gen(),
            ..Default::default()
        };

        for n in 0..device_desc.num_configurations() {
            let config_desc = match device.config_descriptor(n) {
                Ok(c) => c,
                Err(_) => continue,
            };

            for interface in config_desc.interfaces() {
                for interface_desc in interface.descriptors() {
                    for endpoint_desc in interface_desc.endpoint_descriptors() {
                        match endpoint_desc.direction() {
                            Direction::In => usb_reader.ep_in_addr = endpoint_desc.address(),
                            Direction::Out => usb_reader.ep_out_addr = endpoint_desc.address()
                        }
                    }
                }
            }
        }
        usb_reader.device = Some(device);
        usb_readers.push(Arc::new(Mutex::new(usb_reader)));
    }

    usb_readers
}