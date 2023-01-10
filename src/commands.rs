//use std::sync::{Arc, Mutex};

use crate::reader::*;
use crate::errors::*;

pub struct UemCommands<'a> {
    // reader: &'a Arc<Mutex<UemReader<T>>>,
    reader: &'a UemReader,
}

pub trait UemCommandsTrait {
    //fn run(&self);
    fn commands(&mut self) -> UemCommands;
}

// impl ArcMutexUemReaderCommands for Arc<Mutex<UemReader<T>>> {
impl UemCommandsTrait for UemReader {
    //fn run(&self) {
    //    println!("run!");
    //}
    
    fn commands(&mut self) -> UemCommands {
        UemCommands::new(self)
    }

}

impl<'a> UemCommands<'a> {
    // fn new(rd: &'a Arc<Mutex<UemReader<T>>>) -> Self {
    fn new(rd: &'a UemReader) -> Self {
        UemCommands {reader: rd}
    }
    
    pub(crate) fn get_reader_ref(&self) -> &'a UemReader {
        self.reader
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

