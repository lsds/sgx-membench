//! EnclaveLogger

extern crate enclave;

use collections::String;

static mut LOG_BUF: &'static mut [u8] = &mut [];

pub struct EnclaveLogger;

impl EnclaveLogger {
    pub fn init(log_ptr : u64) -> () {
        unsafe {
            LOG_BUF = &mut *(log_ptr as *mut [u8; 255]);
        }
    }
    
    #[allow(dead_code)]
    pub fn log(s : String) -> () {
        let src = s.as_bytes();
        unsafe {
            let dst = &mut LOG_BUF;
            let len = ::core::cmp::min(dst.len(), src.len());
            (&mut dst[..len]).clone_from_slice(&src[..len]);
            enclave::usercall::do_usercall(1, len as u64, 0, 0, 0);
        }
    }
}

macro_rules! enc_log {
    ($($arg:tt)*) => (EnclaveLogger::log(format!($($arg)*)))
}
