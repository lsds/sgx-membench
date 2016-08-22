#![no_std]
#![feature(asm)]

pub mod bench;

pub enum Ecalls { 
    LogInit = 1, 
    NoOp = 2, 
    VecInit = 3, 
    MemAccessSeqBytewise = 4,
    MemAccessSeqPagewise = 5,
    MemAccessRandBytewise = 6
}
