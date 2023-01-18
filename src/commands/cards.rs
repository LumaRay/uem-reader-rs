//! General commands to interact with RFID tags

#![allow(dead_code)]

pub mod mifare;

use crate::reader::*;
use crate::commands::cards::mifare::*;
use crate::errors::*;
use crate::card::*;

#[derive(Debug, Clone, Copy)]
/// Card activation parameters
/// 
/// This structure can be used to both activate cards 
/// of type ISO14443A and ISO14443B
pub struct UemActivateParameters {
    // standard: UemCardStandard,
    /// Required [baudrate](UemCardBaudrates) of card -> reader channel
    pub baudrate_card_reader: UemCardBaudrates,
    /// Required [baudrate](UemCardBaudrates) of reader -> card channel
    pub baudrate_reader_card: UemCardBaudrates,
    /// Time in milliseconds to turn radio off before
    /// requesting next card.
    pub radio_off_period: u8,
    /// After a radio field has been turned on,
    /// it is necessary to give cards little time
    /// to fully power up. Set in milliseconds.
    pub pause_after_radio_on: u8,
    /// Enables cards to be automatically switched to 
    /// T=CL protocol (ISO14443-4)
    pub switch_to_tcl: bool,
    /// If switched to T=CL, then which card identifier
    /// (CID) to use
    pub tcl_cid: u8,
    /// For cards of type ISO14443B - 
    /// application family identifier
    pub btype_afi: u8,
    /// For cards of type ISO14443B - 
    /// request extended answer to query data
    pub btype_use_ext_atqb: bool,
    /// For cards of type ISO14443B - 
    /// number of time slots to use
    pub btype_time_slots: u8,
}

impl Default for UemActivateParameters {
    fn default() -> UemActivateParameters {
        UemActivateParameters {
            // standard: UemCardStandard::Iso14443a,
            baudrate_card_reader: UemCardBaudrates::Baud106kbps,
            baudrate_reader_card: UemCardBaudrates::Baud106kbps,
            radio_off_period: 10,
            pause_after_radio_on: 10,
            switch_to_tcl: false,
            tcl_cid: 0x00,
            btype_afi: 0x00,
            btype_use_ext_atqb: false,
            btype_time_slots: 1,
        }
    }
}

/// Structure for commands to interact
/// with cards
pub struct UemCommandsCards<'a> {
    reader: &'a UemReader,
}

/// Accessing cards related commands group
pub trait UemCommandsCardsTrait {
    fn cards(&mut self) -> UemCommandsCards;
}

impl<'a> UemCommandsCardsMifareTrait for UemCommandsCards<'a> {   
    fn mifare(&mut self) -> UemCommandsCardsMifare {
        UemCommandsCardsMifare::new(self.as_reader())
    }
}

impl<'a> UemCommandsCards<'a> {
    pub(crate) fn new(rd: &'a UemReader) -> Self {
        UemCommandsCards {reader: rd}
    }

    pub(crate) fn as_reader(&self) -> &'a UemReader {
        self.reader
    }

    /// Activation of type ISO14443A card
    /// 
    /// # Arguments
    ///
    /// * `parameters` - A reference to a set of [parameters](UemActivateParameters) to tweak activation,
    /// 
    /// # Returns
    /// 
    /// `Ok(())` on success, otherwise returns an error.
    /// 
    /// # Example
    /// ```ignore
    /// let card = uem_reader.commands().cards().activate_a(&UemActivateParameters{
    ///     // switch_to_tcl: true, // Can be used to set T=CL protocol after activation
    ///     ..Default::default()
    /// });
    /// ```
    pub fn activate_a(&mut self, parameters: &UemActivateParameters) -> UemResultCardA {
        let mut raw_reader = self.reader.lock().unwrap();
        let mut type_baud: u8 = 0x00;
        type_baud |= (parameters.baudrate_card_reader as u8) << 2;
        type_baud |= parameters.baudrate_reader_card as u8;
        let mut rf_reset: u8 = 0x00;
        rf_reset |= (parameters.radio_off_period & 0x0F) << 4;
        rf_reset |= parameters.pause_after_radio_on & 0x0F;
        let mut disable_tcl_cid: u8 = 0x00;
        disable_tcl_cid |= (!parameters.switch_to_tcl as u8) << 7;
        disable_tcl_cid |= parameters.tcl_cid & 0x0F;

        let res = raw_reader.send(&vec![0x75, 
            type_baud, 
            rf_reset, 
            disable_tcl_cid])?;
        
        if res.len() < 8 {
            return Err(UemError::ReaderIncorrectResponse);
        }

        let atq = res[0..2].to_vec();
        let sak: u8 = res[2];
        let uid_len = res[3];
        let uid = res[4 .. 4 + uid_len as usize].to_vec();
       
        if res.len() == 4 + uid_len as usize {
            return Ok(UemCardIso14443A{atq, sak, uid, ats: vec![]});
        }

        let ats_len = res[4 + uid_len as usize];
        let ats = res[4 + uid_len as usize .. 4 + (uid_len + ats_len) as usize].to_vec();

        Ok(UemCardIso14443A{atq, sak, uid, ats})
    }

    /// Activation of type ISO14443B card
    /// 
    /// # Arguments
    ///
    /// * `parameters` - A set of [parameters](UemActivateParameters) to tweak activation,
    /// 
    /// # Returns
    /// 
    /// `Ok(())` on success, otherwise returns an error.
    /// 
    /// # Example
    /// ```ignore
    /// let card = uem_reader.commands().cards().activate_b(&UemActivateParameters{
    ///     // switch_to_tcl: true, // Can be used to set T=CL protocol after activation
    ///     ..Default::default()
    /// });
    /// if card.is_err() {
    ///     uem_reader.close();
    ///     return;
    /// }
    /// let card = card.unwrap();
    /// ```
    pub fn activate_b(&mut self, parameters: &UemActivateParameters) -> UemResultCardB {
        let mut raw_reader = self.reader.lock().unwrap();
        let mut type_baud: u8 = 0b_0001_0000;
        type_baud |= (parameters.baudrate_card_reader as u8) << 2;
        type_baud |= parameters.baudrate_reader_card as u8;
        let mut rf_reset: u8 = 0x00;
        rf_reset |= (parameters.radio_off_period & 0x0F) << 4;
        rf_reset |= parameters.pause_after_radio_on & 0x0F;
        let mut disable_tcl_cid: u8 = 0x00;
        disable_tcl_cid |= (parameters.switch_to_tcl as u8) << 7;
        disable_tcl_cid |= parameters.tcl_cid & 0x0F;
        let mut param: u8 = 0x00;
        param |= (parameters.btype_use_ext_atqb as u8) << 4;
        param |= parameters.btype_time_slots & 0x07;

        let res = raw_reader.send(&vec![0x75, 
            type_baud, 
            rf_reset, 
            disable_tcl_cid, 
            parameters.btype_afi, 
            param])?;

        if res.len() < 2 {
            return Err(UemError::ReaderIncorrectResponse);
        }

        let mbli = res[0];
        let atqb_len = res[1];

        if atqb_len < 12 {
            return Err(UemError::ReaderIncorrectResponse);
        }

        let atqb = res[2 .. 2 + atqb_len as usize].to_vec();
        let pupi = res[3..7].to_vec();
        let app_data = res[7..11].to_vec();
        let prot_info = res[11..14].to_vec();
        
        Ok(UemCardIso14443B{
            mbli, 
            pupi,
            app_data,
            prot_info,
            atq: atqb,
        })
    }
}
