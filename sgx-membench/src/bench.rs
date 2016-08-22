
use core::num::Wrapping;

const TOTAL_MEM_ACCESSED: usize = 256 * 1024 * 1024;

pub fn mem_access_seq(a: &[u8], step: usize) -> u32 {
	let mut sum: u32 = 0;
	let mut i = 0usize;
	let size = a.len();
	while i < TOTAL_MEM_ACCESSED {
		sum = sum.wrapping_add(a[i % size] as u32); // disable overflow checking
		i += step;
	}
	sum	
}

/* Inlined from the Rust Rand implementation */

#[allow(missing_copy_implementations)]
#[derive(Clone)]
struct XorShift {
    x: Wrapping<u32>,
    y: Wrapping<u32>,
    z: Wrapping<u32>,
    w: Wrapping<u32>,
}

impl XorShift {
    fn new() -> XorShift {
        XorShift {
            x: Wrapping(0x193a6754),
            y: Wrapping(0xa8a7d469),
            z: Wrapping(0x97830e05),
            w: Wrapping(0x113ba7bb),
        }
    }
    
    #[inline]
    fn next(&mut self) -> u32 {
	    let x = self.x;
	    let t = x ^ (x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        let w_ = self.w;
        self.w = w_ ^ (w_ >> 19) ^ (t ^ (t >> 8));
        self.w.0
    }
}

pub fn mem_access_rand(a: &[u8]) -> u32 {
	let mut sum: u32 = 0;

	let mut accessed_size = 0usize;
	let size = a.len() as u32;
	
	let mut rng = XorShift::new();
	
	while accessed_size < TOTAL_MEM_ACCESSED {
		let i = (rng.next() % size) as usize; 
		sum = sum.wrapping_add(a[i] as u32);
		accessed_size += 1;
	}
	sum
}
