//! Readers module contains type definitions 
//! for reader operation objects

pub mod usb;
pub mod com;

use crate::errors::*;
use crate::commands::*;
use crate::card::*;
pub use crate::reader::usb::find_usb_readers;

use std::sync::{Arc, Mutex};
use std::{time::Duration};

pub(crate) const TIMEOUT: Duration = Duration::from_secs(1);

/// General reader type using Arc standard type
pub type UemReader = Arc<Mutex<dyn UemReaderInternalTrait+Send>>;
/// Vector of readers discovered using specified method
//pub type UemReaders = Vec<UemReader>;

type UemGeneralResult<T> = core::result::Result<T, UemError>;
/// Common library result
pub type UemResult = UemGeneralResult<()>;
/// Library result containing returned vector of bytes
pub type UemResultVec = UemGeneralResult<Vec<u8>>;
/// Library result containing ISO14443a card
pub type UemResultCardA = UemGeneralResult<UemCardIso14443A>;
/// Library result containing ISO14443b card
pub type UemResultCardB = UemGeneralResult<UemCardIso14443B>;

impl UemCommandsTrait for UemReader {   
    fn commands(&mut self) -> UemCommands {
        UemCommands::new(self)
    }
}

/// Common reader methods
pub trait UemReaderInternalTrait {
    fn open(&mut self) -> UemResult;
    fn close(&mut self) -> core::result::Result<(), UemError>;
    fn send(&mut self, command: &[u8]) -> UemResultVec;
}

impl UemReaderInternalTrait for UemReader {
    /// Open interface with the reader
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// if uem_reader.open().is_err() {
    ///     return;
    /// }
    /// ```
    fn open(&mut self) -> UemResult {
        self.lock().unwrap().open()
    }

    /// Close opened reader interface 
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// if uem_reader.close().is_err() {
    ///     return;
    /// };
    /// ```
    fn close(&mut self) -> core::result::Result<(), UemError> {
        self.lock().unwrap().close()
    }

    /// Send a command to a reader and receive response
    /// 
    /// # Arguments
    ///
    /// * `command` - a vector of command bytes
    /// 
    /// # Returns
    /// 
    /// `Ok(())` on success, otherwise returns an error.
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// // Beep 1 time using command byte vector
    /// if uem_reader.send(&vec![0x05_u8, 0x01_u8]).is_err() {
    ///     return;
    /// }
    /// ```
    fn send(&mut self, command: &[u8]) -> UemResultVec {
        self.lock().unwrap().send(command)
    }
}

pub(crate) mod processing {
    use crate::{helpers::*, reader::*};
    pub(crate) trait CommandsCounter {
        fn commands_count(&self) -> u8;
        fn increment_commands(&mut self);
    }

    pub(crate) fn prepare_command(reader: &mut impl CommandsCounter, data: &[u8]) -> Vec<u8> {
        
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
        raw_data.append(&mut fsc);

        let mut tmp_data = byte_stuff(&raw_data);
        let mut raw_data: Vec<u8> = vec![];
        raw_data.reserve(2 + tmp_data.len());
        raw_data.push(0xFD);
        raw_data.append(&mut tmp_data);
        raw_data.push(0xFE);
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
        let fsc = crc16(&raw_data[1..raw_data.len()-3]);
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
}