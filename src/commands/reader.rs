//! Commands relative to control behavior of
//! a reader itself

use crate::reader::*;
use crate::errors::*;

/// Structure for commands controlling 
/// a reader itself
pub struct UemCommandsReader<'a> {
    reader: &'a UemReader,
}

/// Accessing reader related commands group
pub trait UemCommandsReaderTrait {
    fn reader(&mut self) -> UemCommandsReader;
}

impl<'a> UemCommandsReader<'a> {
    pub(crate) fn new(rd: &'a UemReader) -> Self {
        UemCommandsReader {reader: rd}
    }

    /// Make signals of specific count
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// // Beep 5 times using command grouping objects as separate variables
    /// let mut uem_cmds = uem_reader.commands();
    /// let mut uem_cmds_reader = uem_cmds.reader();
    /// if uem_cmds_reader.beep(5).is_err() {
    ///     return;
    /// }
    /// ```
    pub fn beep(&mut self, count: i32) -> core::result::Result<(), UemError> {
        if count < 1 || count > 255 {
            return Err(UemError::IncorrectParameter);
        }
        let mut raw_reader = self.reader.lock().unwrap();
        raw_reader.send(vec![0x05_u8, count as u8]).map(|_| ())
    }
}
