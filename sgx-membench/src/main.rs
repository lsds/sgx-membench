extern crate sgx_membench;
extern crate sgxs;
extern crate sgx_isa;
extern crate enclave_interface;
extern crate x86;
extern crate number_prefix;

pub mod logger;

use std::fs::File;
use std::str;
use std::env;
use sgxs::loader::{Load, Map};
use logger::Logger;
use sgx_membench::bench::{mem_access_seq, mem_access_rand};
use sgx_membench::Ecalls;
use x86::bits64::time;
use std::io;
use std::io::Write;
use number_prefix::{binary_prefix, Standalone, Prefixed};

const NUM_RUNS: u64 = 3; // was: 10
const NOOP_RUNS: u64 = 5; // was: 25
const PAGE_SIZE: usize = 4096;
const BUILT: &'static str = "debug";

macro_rules! measure_avg {
    ($runs:ident, $f:stmt) => {{
		let mut duration : u64 = 0;
	    for n in 1..($runs + 2) {
			let start_time = unsafe { time::rdtscp() };
			//println!("calling f run={}", n);
			$f;
			let stop_time = unsafe { time::rdtscp() };
			//println!("n={} time={}", n, stop_time - start_time);
			if n > 1 {
				duration += stop_time - start_time;
			}
	    }
	    //println!("duration={}", duration);
		duration / $runs
    }}
}

fn init_enclave<'a> (dev : &'a sgxs::isgx::Device) -> (sgxs::isgx::Mapping<'a>, std::result::Result<enclave_interface::debug::InstalledSignalHandler, std::io::Error>) {
    let (mut file, sig, mut le_file, le_sig) = (File::open(format!("../sgx-membench-enclave/target/{}/sgx_membench_enclave.sgxs", BUILT)).unwrap(),
                                        format!("../sgx-membench-enclave/target/{}/sgx_membench_enclave.sig", BUILT),
                                        File::open("../rust-sgx/enclave_init/le.sgxs").unwrap(),
                                        "../rust-sgx/enclave_init/le_prod_css.bin");
    
    let mut sig_file = File::open(sig).unwrap();
    let sig = enclave_interface::util::read_sigstruct(&mut sig_file).unwrap();    
    let mut le_sig_file = File::open(le_sig).unwrap();
    let le_sig = enclave_interface::util::read_sigstruct(&mut le_sig_file).unwrap();    
    let mut mapping = dev.load_with_launch_enclave(&mut file, &sig, sgxs::loader::OptionalEinittoken::None(None), &mut le_file, &le_sig).unwrap();
    let h = enclave_interface::debug::install_segv_signal_handler(&mut mapping.tcss()[0]);
	(mapping, h)
}

fn ecall<F>(mapping: &mut sgxs::isgx::Mapping, ocall_handler: F, nr: u64, p1: u64, p2: u64, p3: u64, p4: u64) -> u64
	where F : FnMut(u64, u64, u64, u64, u64) -> u64 {
	
	enclave_interface::tcs::enter(&mut mapping.tcss()[0], ocall_handler, nr, p1, p2, p3, p4)
}

fn main() {
	let dev = sgxs::isgx::Device::open("/dev/isgx").unwrap();	
    let (mut mapping, h) = init_enclave(&dev);

	let args: Vec<String> = env::args().collect();    
    let mem_size : usize = args[1].parse().unwrap();

	match binary_prefix(mem_size as f64) {
	    Standalone(bytes)   => print!("# mem_size={} bytes ", bytes),
	    Prefixed(prefix, n) => print!("# mem_size={:.0} {}B ", n, prefix),
	}
	println!("NUM_RUNS={}", NUM_RUNS);
        
    //let mut msg_buf = [0u8; 255];    
    //let l = Logger::new(&mut msg_buf);    
                    
	println!("# mem_size out_seq_bytewise out_seq_pagewise out_rand_bytewise enc_seq_bytewise enc_seq_pagewise enc_rand_bytewise");
	print!("{} ", mem_size);
	io::stdout().flush().unwrap();

    let a:Vec<u8> = vec![170u8; mem_size];
    
    let time_out_seq_bytewise = measure_avg!(NUM_RUNS, mem_access_seq(&a, 1));
    print!("{} ", time_out_seq_bytewise);
	io::stdout().flush().unwrap();

    let time_out_seq_pagewise = measure_avg!(NUM_RUNS, mem_access_seq(&a, PAGE_SIZE));
    print!("{} ", time_out_seq_pagewise);
	io::stdout().flush().unwrap();
    
    let time_out_rand = measure_avg!(NUM_RUNS, mem_access_rand(&a));
    print!("{} ", time_out_rand);    
	io::stdout().flush().unwrap();
    	        
    let ocall_handler = | nr, p1, _p2, _p3, _p4 | {
		//println!("Usercall: nr={} size={} {} {} {}", nr, p1, p2, p3, p4)
		match nr {
			//1 => l.display(p1),
			_ => panic!("unknown ocall {}", nr),
		}
	    0	    
    };
        
    // log init    
    //ecall(&mut mapping, &ocall_handler, Ecalls::LogInit as u64, l.get_raw_ptr(), 0, 0, 0);
  
    // no-op measurement
    let time_avg_ecall_noop = measure_avg!(NOOP_RUNS, ecall(&mut mapping, &ocall_handler, Ecalls::NoOp as u64, 0, 0, 0, 0));
    //println!("time_avg_ecall_noop={}", time_avg_ecall_noop);
        
    // init encl array
    let a_ptr = ecall(&mut mapping, &ocall_handler, Ecalls::VecInit as u64, 0, mem_size as u64, 0, 0);
    
    let time_encl_seq_bytewise: u64 = measure_avg!(NUM_RUNS, ecall(&mut mapping, &ocall_handler, Ecalls::MemAccessSeqBytewise as u64, a_ptr, mem_size as u64, 0, 0)).saturating_sub(time_avg_ecall_noop);
    print!("{} ", time_encl_seq_bytewise);
	io::stdout().flush().unwrap();

    let time_encl_seq_pagewise: u64 = measure_avg!(NUM_RUNS, ecall(&mut mapping, &ocall_handler, Ecalls::MemAccessSeqPagewise as u64, a_ptr, mem_size as u64, 0, 0)).saturating_sub(time_avg_ecall_noop);
    print!("{} ", time_encl_seq_pagewise);
	io::stdout().flush().unwrap();
    
    let time_encl_rand_bytewise: u64 = measure_avg!(NUM_RUNS, ecall(&mut mapping, &ocall_handler, Ecalls::MemAccessRandBytewise as u64, a_ptr, mem_size as u64, 0, 0)).saturating_sub(time_avg_ecall_noop);
    println!("{}", time_encl_rand_bytewise);    
        
    drop(h);
}
