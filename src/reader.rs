use crate::errors::*;
use crate::helpers::*;
use crate::commands::*;

use rusb::{
    /*ConfigDescriptor, DeviceDescriptor,*/ DeviceHandle, DeviceList, /*EndpointDescriptor,*/
    /*InterfaceDescriptor,*/ Language, Result, Device, /*Speed,*/ UsbContext, Direction, GlobalContext,
};

use std::{time::Duration};//, fmt::Error, thread};

use rand::Rng;

use std::cell::RefCell;
use std::rc::{Rc, Weak};

// #![doc = include_str!("../README.md")]

const UEM_VID: u16 = 0xC251;
const UEM_PID: u16 = 0x130A;

const TIMEOUT: Duration = Duration::from_secs(1);

type UemReaders = Vec<UemReader<GlobalContext>>;

#[derive(Default)]
pub struct UemReader<T: UsbContext> {
    handle: Option<DeviceHandle<T>>,
    device: Option<Device<T>>,
    language: Option<Language>,
    timeout: Duration,
    ep_in_addr: u8,
    ep_out_addr: u8,
    ncommand: u8,
    // pub commands: Vec<Rc<RefCell<Commands<T>>>>,
    pub commands: Box<Commands<T>>,
}

impl<T: UsbContext> UemReader<T> {
    #![warn(missing_docs)]
    pub fn open(&mut self) -> UemResult {
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
    pub fn close(&mut self) -> core::result::Result<(), UemError> {
        if self.handle.is_none() {
            return Err(UemError::ReaderNotConnected);
        }
        if let Some(h) = self.handle.take() {
            self.device = Some(h.device());
            return Ok(())
        }
        return Ok(())
    }

    pub fn transceive(&mut self, command: Vec<u8>) -> UemResultVec {
        
        if self.handle.is_none() {
            return Err(UemError::ReaderNotConnected);
        }
        if command.is_empty() {
            return Err(UemError::IncorrectParameter);
        }

        // int TIMEOUT = 0;
        let send_buffer = self.wrap_command(&command);
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

        let response = self.unwrap_response(&receive_buffer[..response_length].to_vec())?;

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

    fn wrap_command(&mut self, data: &Vec<u8>) -> Vec<u8> {
    
        let mut raw_data: Vec<u8> = vec![];
    
        raw_data.push(0x00);
        raw_data.push(self.ncommand);
        if self.ncommand == u8::MAX {
            self.ncommand = 0;
        }
        self.ncommand += 1;
        //if ((reader != null) && reader.Reader.encryptedMode) {
        //    rawData.write(0x00);
        //    data = AES.encryptChannel(data, reader);
        //    if (data == null)
        //        return null;
        //}
        let mut tmp_v = vec![];
        data.clone_into(&mut tmp_v);
        raw_data.append(&mut tmp_v);
    
        let mut fsc = crc16(&raw_data);
        //fsc.clone_into(&mut tmp_v);
        raw_data.append(&mut fsc);
    
        let mut tmp_data = byte_stuff(&raw_data);
        let mut raw_data: Vec<u8> = vec![];
        raw_data.reserve(2 + tmp_data.len());
        raw_data.push(0xFD);
        raw_data.append(&mut tmp_data);
        //raw_data.reserve(2);
        raw_data.push(0xFE);
        println!("{:?}", raw_data);
        return raw_data;
    }
    
    fn unwrap_response(&mut self, raw_data: &Vec<u8>) -> UemResultVec {
        let raw_data = unbyte_stuff(raw_data);
        if (raw_data[0] & 0xFF) != 0xFD {
            return Err(UemError::ReaderUnsuccessful(UemInternalError::Protocol, None));
        }
        if (raw_data[raw_data.len()-1] & 0xFF) != 0xFE {
            return Err(UemError::ReaderUnsuccessful(UemInternalError::Protocol, None));
        }
        let fsc = crc16(&raw_data[1..raw_data.len()-3].to_vec());
        if (fsc[0] & 0xFF) != (raw_data[raw_data.len()-3] & 0xFF) {
            return Err(UemError::ReaderUnsuccessful(UemInternalError::Crc, None));//  Err(UemError::CRC);
        }
        if  (fsc[1] & 0xFF) != (raw_data[raw_data.len()-2] & 0xFF) {
            return Err(UemError::ReaderUnsuccessful(UemInternalError::Crc, None));
        }
        let data = raw_data[3..raw_data.len()-3].to_vec();
        //if (reader != null) && reader.Reader._encryptedMode && (data[0] == 0x00) {
        //    data = AES.decryptChannel(Arrays.copyOfRange(data, 1, data.length), reader);
        //}
        return Ok(data);
    }
}

pub fn find_readers() -> UemReaders {
    let mut uem_readers: UemReaders = Vec::new();
    let devices = DeviceList::new();
    if let Err(_) = devices {
        return uem_readers;
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

        let mut uem_reader = UemReader {
            ncommand: rand::thread_rng().gen(),
            ..Default::default()
        };
        // uem_reader.commands = vec![Rc::new(RefCell::new(Commands{reader: Rc::downgrade(&Rc::new(RefCell::new(uem_reader)))}))];
        // uem_reader.commands = Commands{reader: std::ptr::null()};
        uem_reader.commands = Box::new(Commands{reader: &mut uem_reader});

        for n in 0..device_desc.num_configurations() {
            let config_desc = match device.config_descriptor(n) {
                Ok(c) => c,
                Err(_) => continue,
            };

            for interface in config_desc.interfaces() {
                for interface_desc in interface.descriptors() {
                    for endpoint_desc in interface_desc.endpoint_descriptors() {
                        match endpoint_desc.direction() {
                            Direction::In => uem_reader.ep_in_addr = endpoint_desc.address(),
                            Direction::Out => uem_reader.ep_out_addr = endpoint_desc.address()
                        }
                    }
                }
            }
        }
        uem_reader.device = Some(device);
        uem_readers.push(uem_reader);       
    }

    uem_readers
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        let readers = find_readers();
        assert!(!readers.is_empty());
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}