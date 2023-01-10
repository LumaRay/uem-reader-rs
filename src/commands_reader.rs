use crate::commands::*;
//use crate::commands_reader::*;
use crate::reader::*;
use crate::errors::*;

pub struct UemCommandsReader<'a> {
    reader: &'a UemReader,
}

pub trait UemCommandsReaderTrait {
    //fn run(&self);
    fn reader(&mut self) -> UemCommandsReader;
}

impl<'a> UemCommandsReaderTrait for UemCommands<'a> {
    //fn run(&self) {
    //    println!("run!");
    //}
    
    fn reader(&mut self) -> UemCommandsReader {
        UemCommandsReader::new(self.get_reader_ref())
    }
}

impl<'a> UemCommandsReader<'a> {
    // fn new(rd: &'a Arc<Mutex<UemReader<T>>>) -> Self {
    fn new(rd: &'a UemReader) -> Self {
        UemCommandsReader {reader: rd}
    }
    
    // fn toast(&self) {
    //     self.reader.lock().unwrap().count();
    //     println!("toasting! {:?}", self.reader.lock().unwrap().counter);
    // }

    pub fn beep(&mut self, count: i32) -> core::result::Result<(), UemError> {
        if count < 1 || count > 255 {
            return Err(UemError::IncorrectParameter);
        }
        let mut raw_reader = self.reader.lock().unwrap();
        raw_reader.send(vec![0x05_u8, count as u8]).map(|_| ())
    }
}
