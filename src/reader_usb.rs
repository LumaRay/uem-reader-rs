use std::sync::{Arc, Mutex};

use rusb::{
    DeviceHandle, DeviceList, Language, 
    Device, UsbContext, Direction,
};

use std::{time::Duration};

use rand::Rng;

//use crate::{reader::{CommandsCounter, UemReaders, UemReaderInternal, TIMEOUT, prepare_command, parse_response}, errors::{UemResult, UemError, UemInternalError, UemResultVec}};
use crate::reader::*;
//use crate::reader_usb::find_usb_readers as _find_usb_readers;
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
//    counter: i32,
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

pub fn find_usb_readers() -> UemReaders {
    let mut usb_readers: UemReaders = Vec::new();
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
        // uem_reader.commands = vec![Rc::new(RefCell::new(Commands{reader: Rc::downgrade(&Rc::new(RefCell::new(uem_reader)))}))];
        // uem_reader.commands = Commands{reader: std::ptr::null()};
        //uem_reader.commands = Box::new(Commands{reader: &mut uem_reader});

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

// impl<T: UsbContext> ReaderUsb<T> {
impl<T: UsbContext> UemReaderInternal for ReaderUsb<T> {
    #![warn(missing_docs)]
    fn open(&mut self) -> UemResult {
        if self.handle.is_some() {
            return Err(UemError::ReaderAlreadyConnected);
        }
        // if let Some(mut uem_reader) = uem_readers.get_mut(0) {
            //usb_device.handle = usb_device.device.take().unwrap().open().ok();
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
        // }
        Err(UemError::ReaderConnectionFailed)
    }        

    /// close opened USB interface
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

    fn send(&mut self, command: Vec<u8>) -> UemResultVec {
        
        if self.handle.is_none() {
            return Err(UemError::ReaderNotConnected);
        }
        if command.is_empty() {
            return Err(UemError::IncorrectParameter);
        }

        // int TIMEOUT = 0;
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

        Ok(response)
    }
}