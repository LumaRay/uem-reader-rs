use crate::reader::*;
use crate::errors::*;

//use std::sync::Weak;
use std::cell::RefCell;
use std::pin::Pin;
use std::rc::{Rc, Weak};

use rusb::UsbContext;

use std::borrow::Borrow;

pub struct Commands<T: UsbContext> {
    // reader: Weak<RefCell<UemReader<T>>>,
    pub reader: *mut UemReader<T>,
}

impl<T: UsbContext> Default for Commands<T> {
    fn default() -> Self {
        Self { reader: std::ptr::null_mut() }
    }
}

impl<T: UsbContext> Commands<T> {
    /// Make short beep sound
    ///
    /// # Examples
    ///
    /// //```rust
    /// //assert_eq!(min( 0,   14),    0);
    /// //assert_eq!(min( 0, -127), -127);
    /// //assert_eq!(min(42,  666),   42);
    /// //```
    pub fn beep(self, count: i32) -> core::result::Result<(), UemError> {
        if count < 1 || count > 255 {
            return Err(UemError::IncorrectParameter);
        }
        // let raw_reader: RefCell<UemReader<T>> = unsafe {*self.reader.into_raw()};
        // raw_reader.borrow_mut().transceive(vec![0x05_u8, 0x01_u8]).map(|_| ())
        // let raw_reader = unsafe {*self.reader};
        let raw_reader = unsafe{ &mut *self.reader };
        raw_reader.transceive(vec![0x05_u8, count as u8]).map(|_| ())
    }
}


































/*
//use std::sync::Arc;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct Commands<'a> {
    toaster: &'a Arc<Mutex<Toaster>>,
}

impl<'a> Commands<'a> {
    fn new(tr: &'a Arc<Mutex<Toaster>>) -> Self {
        Commands {toaster: tr}
    }
    
    fn toast(&self) {
        self.toaster.lock().unwrap().count();
        println!("toasting! {:?}", self.toaster.lock().unwrap().counter);
    }
}

trait MyRunner {
    //fn new() -> Self;
    fn run(&self);
    fn commands(&self) -> Commands;
}

#[derive(Debug, Default)]
struct Toaster {
    counter: i32,
    //cmds: Option<Commands>,
}

impl Toaster {
    fn new() -> Arc<Mutex<Toaster>> {
        let am_toaster = Arc::new(Mutex::new(Self{..Default::default()}));
//        am_toaster.lock().unwrap().cmds = Some(Commands { toaster: am_toaster.clone() });
        am_toaster
    }
    
    fn count(&mut self) {
        self.counter += 1;
    }
}

impl MyRunner for Arc<Mutex<Toaster>> {
    //fn new() -> Self {
    //    Toaster::new()
    //}
    
    fn run(&self) {
        println!("run!");
    }
    
    fn commands(&self) -> Commands {
        Commands::new(self)
    }
}

fn main() {
    let tmp = Toaster::new();
    //let tmp2 = tmp::new();
    println!("Hello, world!");
    
    tmp.run();
    
    //tmp.lock().unwrap().cmds.as_ref().unwrap().toast();
    
    tmp.commands().toast();
    tmp.commands().toast();
    tmp.commands().toast();
    tmp.commands().toast();
    
    let tc = tmp.commands();
    tc.toast();
    tc.toast();
    tc.toast();
}
*/
