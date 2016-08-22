#![no_std]
#![feature(collections)]

extern crate enclave;
#[macro_use] extern crate core_io;
#[macro_use] extern crate collections;
extern crate sgx_membench;

#[macro_use] mod enclave_logger;
use enclave_logger::EnclaveLogger;
use sgx_membench::bench::{mem_access_seq, mem_access_rand};
use sgx_membench::Ecalls;
use collections::Vec;
use core::mem;

const PAGE_SIZE: usize = 4096;

#[no_mangle]
pub extern "C" fn entry(nr: u64, p1: u64, p2: u64, _ignore: u64, _p3: u64, _p4: u64) -> u64 {	
	match unsafe { mem::transmute(nr as u8) } {
		Ecalls::LogInit => { 
			EnclaveLogger::init(p1);
			//enc_log!("hello from the enclave ptr={}", p1);
			0
		},
		Ecalls::NoOp => { 
			0 
		},
		Ecalls::VecInit => {
			let a: Vec<u8> = vec![170u8; p2 as usize];
			let ptr: u64 = unsafe { mem::transmute(a.as_ptr()) };
			//enc_log!("ecall3 ptr={} p2={} a={:?}", ptr, p2, a);
			mem::forget(a);
			ptr
		},
		Ecalls::MemAccessSeqBytewise => {
			//enc_log!("ecall4 p1={} p2={}", p1, p2);
		    let a: Vec<u8> = unsafe { Vec::from_raw_parts(mem::transmute(p1), p2 as usize, p2 as usize) };
		    //enc_log!("ecall4 a={:?}", a);
		    let sum = mem_access_seq(&a, 1);
		    mem::forget(a); 
			//enc_log!("timing={}", out_seq);
			sum as u64
		}
		Ecalls::MemAccessSeqPagewise => {
			//enc_log!("ecall4 p1={} p2={}", p1, p2);
		    let a: Vec<u8> = unsafe { Vec::from_raw_parts(mem::transmute(p1), p2 as usize, p2 as usize) };
		    //enc_log!("ecall4 a={:?}", a);
		    let sum = mem_access_seq(&a, PAGE_SIZE);
		    mem::forget(a); 
			//enc_log!("timing={}", out_seq);
			sum as u64
		}
		Ecalls::MemAccessRandBytewise => {
			//enc_log!("ecall4 p1={} p2={}", p1, p2);
		    let a: Vec<u8> = unsafe { Vec::from_raw_parts(mem::transmute(p1), p2 as usize, p2 as usize) };
		    //enc_log!("ecall4 a={:?}", a);
		    let sum = mem_access_rand(&a);
		    mem::forget(a); 
			//enc_log!("timing={}", out_seq);
			sum as u64
		}
		//_ => panic!("unknown ecall {}", nr),
	}
}
