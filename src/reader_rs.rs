#[derive(Debug, Default)]
struct ReaderRs {
    //handle: Option<DeviceHandle<T>>,
    //device: Option<Device<T>>,
    timeout: Duration,
    port: u8,
//    ncommand: u8,
//    counter: i32,
}

impl CommandsCounter for ReaderRs {
    fn commands_count(&self) -> u8 {
        self.ncommands
    }

    fn increment_commands(&self) {
        if self.commands_count() == u8::MAX {
            self.ncommand = 0;
        }
        self.ncommand += 1;
    }
}

pub fn find_rs_readers() -> UemReaders {
    let mut rs_readers: UemReaders = Vec::new();
    rs_readers
}

impl ReaderRs {
    #![warn(missing_docs)]
    pub fn open(&mut self) -> UemResult {
        Ok(())
    }        

    /// close opened RS interface
    pub fn close(&mut self) -> core::result::Result<(), UemError> {
        Ok(())
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
}

