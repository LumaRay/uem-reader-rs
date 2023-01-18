//! Grouping of commands related to Mifare Classic cards type

use crate::{reader::*, card::UemCardIso14443A, helpers::get_absolute_block_address, errors::UemError};

/// Structure for commands to interact
/// with Mifare Classic cards
pub struct UemCommandsCardsMifareClassic<'a> {
    reader: &'a UemReader,
}

/// Accessing Mifare Classic cards related commands group
pub trait UemCommandsCardsMifareClassicTrait {
    fn classic(&mut self) -> UemCommandsCardsMifareClassic;
}

impl<'a> UemCommandsCardsMifareClassic<'a> {
    pub(crate) fn new(rd: &'a UemReader) -> Self {
        UemCommandsCardsMifareClassic {reader: rd}
    }

    /// Authenticate Mifare Classic card with key A
    /// 
    /// # Arguments
    ///
    /// * `card` - A reference to a [card](UemCardIso14443A),
    /// with which to perform the authentication
    /// * `key` - A vector with a 6-bytes key to use
    /// * `sector` - A sector number (0-based) to authenticate
    /// 
    /// # Returns
    /// 
    /// `Ok(())` on success, otherwise returns an error.
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// // Authenticate sector 1 with FF... key
    /// let res = uem_reader.commands().cards().mifare().classic()
    ///     .authenticate_key_a(
    ///         &card, 
    ///         &vec![0xFF; 6], 
    ///         1
    ///     );
    /// if res.is_err() {
    ///     uem_reader.close();
    ///     return;
    /// }
    /// ```
    pub fn authenticate_key_a(&mut self, card: &UemCardIso14443A, key: &[u8; 6], sector: u8) -> UemResult {
        let mut raw_reader = self.reader.lock().unwrap();
        let command :Vec<u8> = vec![0x14, 0x60].iter().cloned().chain(
            card.uid.iter().rev().take(4).rev().cloned().chain(
                key.iter().cloned().chain(
                    vec![get_absolute_block_address(sector, 0)].iter().cloned()
                )
            )
        ).collect();
        raw_reader.send(&command).map(|_| ())
    }

    /// Authenticate Mifare Classic card with key B
    /// 
    /// # Arguments
    ///
    /// * `card` - A reference to a [card](UemCardIso14443B),
    /// with which to perform the authentication
    /// * `key` - A vector with a 6-bytes key to use
    /// * `sector` - A sector number (0-based) to authenticate
    /// 
    /// # Returns
    /// 
    /// `Ok(())` on success, otherwise returns an error.
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// // Authenticate sector 1 with FF... key
    /// let res = uem_reader.commands().cards().mifare().classic()
    ///     .authenticate_key_b(
    ///         &card, 
    ///         &vec![0xFF; 6], 
    ///         1
    ///     );
    /// if res.is_err() {
    ///     uem_reader.close();
    ///     return;
    /// }
    /// ```
    pub fn authenticate_key_b(&mut self, card: &UemCardIso14443A, key: &[u8; 6], sector: u8) -> UemResult {
        let mut raw_reader = self.reader.lock().unwrap();
        let command: Vec<u8> = vec![0x14, 0x61].iter().cloned().chain(
            card.uid.iter().rev().take(4).rev().cloned().chain(
                key.iter().cloned().chain(
                    vec![get_absolute_block_address(sector, 0)].iter().cloned()
                )
            )
        ).collect();
        raw_reader.send(&command).map(|_| ())
    }

    /// Read specific Mifare Classic card block
    /// 
    /// # Arguments
    ///
    /// * `sector` - A sector number (0-based) to use
    /// * `block` - A block number (0-based) within the sector to read
    /// 
    /// # Returns
    /// 
    /// `Ok(Vec<u8>)` with 16-byte block data
    /// on success, otherwise returns an error.
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// // Read sector 1, block 1
    /// let res = uem_reader.commands().cards().mifare().classic()
    ///     .read(1, 1);
    /// if res.is_err() {
    ///     uem_reader.close();
    ///     return;
    /// }
    /// let data = res.unwrap();
    /// ```
    pub fn read(&mut self, sector: u8, block: u8) -> UemResultVec {
        let mut raw_reader = self.reader.lock().unwrap();
        let res = raw_reader.send(&vec![0x19, get_absolute_block_address(sector, block)])?;
        if res.len() != 16 {
            return Err(UemError::ReaderIncorrectResponse);
        }
        Ok(res)
    }

    /// Write to specific Mifare Classic card block
    /// 
    /// # Arguments
    ///
    /// * `data` - 16-bytes vector of data to write to the block
    /// * `sector` - A sector number (0-based) to use
    /// * `block` - A block number (0-based) within the sector to write to
    /// 
    /// # Returns
    /// 
    /// `Ok(())` on success, otherwise returns an error.
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// // Write data to sector 1, block 1
    /// let res = uem_reader.commands().cards().mifare().classic()
    ///     .write(1, 1, &data);
    /// if res.is_err() {
    ///     uem_reader.close();
    ///     return;
    /// }
    /// ```
    pub fn write(&mut self, data: Vec<u8>, sector: u8, block: u8) -> UemResult {
        if data.len() != 16 {
            return Err(UemError::IncorrectParameter);
        }
        let mut raw_reader = self.reader.lock().unwrap();
        let command: Vec<u8> = vec![0x1A, 
        get_absolute_block_address(sector, block)].iter().cloned().chain(data).collect();
        raw_reader.send(&command).map(|_| ())
    }
}