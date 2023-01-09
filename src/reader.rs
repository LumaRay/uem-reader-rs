use crate::errors::*;
use crate::helpers::*;
use crate::commands::*;

use std::sync::{Arc, Mutex};



use std::{time::Duration};

pub(crate) const TIMEOUT: Duration = Duration::from_secs(1);

pub type UemReader = Arc<Mutex<dyn UemReaderInternal>>;
// type UemReaders = Vec<UemReader<GlobalContext>>;
// type UemReaders = Vec<dyn UemReader>;
pub type UemReaders = Vec<UemReader>;

pub trait UemReaderInternal {
    fn open(&mut self) -> UemResult;
    fn close(&mut self) -> core::result::Result<(), UemError>;
    fn transceive(&mut self, command: Vec<u8>) -> UemResultVec;
}

impl UemReaderInternal for UemReader {
    fn open(&mut self) -> UemResult {
        self.lock().unwrap().open()
    }
    fn close(&mut self) -> core::result::Result<(), UemError> {
        self.lock().unwrap().close()
    }
    fn transceive(&mut self, command: Vec<u8>) -> UemResultVec {
        self.lock().unwrap().transceive(command)
    }
}

//#[derive(Debug, Default)]
//struct UemReader<T: UsbContext> {
//    reader: UemReaderTrait,
//    counter: i32,
//}



// impl<T: UsbContext> UemReader<T> {
//     fn new() -> Arc<Mutex<UemReader<T>>> {
//         Arc::new(Mutex::new(Self{..Default::default()}))
//     }
//     //fn count(&mut self) {
//     //    self.counter += 1;
//     //}
// }

pub(crate) trait CommandsCounter {
    fn commands_count(&self) -> u8;
    fn increment_commands(&mut self);
}

pub(crate) fn prepare_command(reader: &mut impl CommandsCounter, data: &Vec<u8>) -> Vec<u8> {
    
    let mut raw_data: Vec<u8> = vec![];

    raw_data.push(0x00);
    raw_data.push(reader.commands_count());
    reader.increment_commands();

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

pub(crate) fn parse_response(raw_data: &Vec<u8>) -> UemResultVec {
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
    Ok(data)
}