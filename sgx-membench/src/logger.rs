//! Logger module

use std::fmt;
use std::str;

//#[derive(Debug)]
pub struct Logger<'a> {
	pub msg_buf: &'a mut [u8],
	pub size : usize
}

impl<'a> fmt::Debug for Logger<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.msg_buf[..].fmt(formatter)
    }
}

impl<'a> Logger<'a> {
	pub fn new(msg_buf : &'a mut [u8]) -> Logger<'a> {
		Logger{ msg_buf : msg_buf, size : 0 }
	}
					
	pub fn get_raw_ptr(&self) -> u64 {
		(&self.msg_buf[0] as *const u8) as u64
	}
	
	pub fn get_size(&self) -> usize { self.size }
	
	pub fn display(&self, size : u64) -> () {
	    let s = str::from_utf8(&self.msg_buf[..size as usize]).unwrap();
		println!("#E: {}", s);
	}
}