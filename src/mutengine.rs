use crate::helpertools::random;
use std::collections::VecDeque;
use crate::fuzzstat::FuzzerStatus;
use crate::config::{ProgConfig, SeedConfig, Stat};

#[derive(Debug, Clone)]
pub struct Mutation {
    pub parent: usize,
    pub mutant: MutType,
}

#[derive(Debug, Clone)]
pub enum MutType {
    BitFlip,
    NibbleMod,
    IntMod,
    AsciiMod,
    HotValues,
    ArithMetic,
    BlockRm,
    BlockInsert,
    BlockSwap,
    None,
}

pub fn mutate(seed_config: &SeedConfig, seed_queue: &VecDeque<SeedConfig>,fuzzstatus: &mut FuzzerStatus) -> SeedConfig {
    let buf = seed_config.seed.clone();
    let buf = match random(1) {
        0 => bit_flip(buf),
        1 => nibble_mod(buf),
         _ => panic!("Unknown"),
    };
//    println!("{:?}",buf);
    SeedConfig::new(buf.clone(),fuzzstatus.conf_count+1)
}

fn bit_flip(buf: Vec<u8>) -> Vec<u8> {
    let mut buf = buf;
    let pos = random(buf.len());
    buf[pos] = match random(1) {
        0 => match random(8) {
            0 => buf[pos]^0b00000001,
            1 => buf[pos]^0b00000010,
            2 => buf[pos]^0b00000100,
            3 => buf[pos]^0b00001000,
            4 => buf[pos]^0b00010000,
            5 => buf[pos]^0b00100000,
            6 => buf[pos]^0b01000000,
            7 => buf[pos]^0b10000000,
            _ => panic!("Unknown"),
        },
        1 => match random(7) {
            0 => buf[pos]^0b00000011,
            1 => buf[pos]^0b00000110,
            2 => buf[pos]^0b00001100,
            3 => buf[pos]^0b00011000,
            4 => buf[pos]^0b00110000,
            5 => buf[pos]^0b01100000,
            6 => buf[pos]^0b11000000,
            _ => panic!("Unknown"),
        },
        2 => match random(6) {
            0 => buf[pos]^0b00000111,
            1 => buf[pos]^0b00001110,
            2 => buf[pos]^0b00011100,
            3 => buf[pos]^0b00111000,
            4 => buf[pos]^0b01110000,
            5 => buf[pos]^0b11100000,
            _ => panic!("Unknown"),
        },
         _ => panic!("Unknown"),
    };
    buf
}

fn nibble_mod(buf: Vec<u8>) -> Vec<u8> {
    let pos = random(buf.len());
    match random(9) {
        0|2|4|6 => match random(2){
            0 => buf[pos]^0b00001111,
            1 => buf[pos]^0b11110000,
            _ => panic!("Unknown"),
        },
        1|3|5 => buf[pos]^0b11111111,
        7|8   => buf[pos]^0b11111111,    //      buf[pos+1]^0b11111111;
        _ => panic!("Unknown"),
       
    };
    buf
}


fn int_mod(buf: Vec<u8>) {}

fn ascii_mod(buf: Vec<u8>) {}

fn hot_values(buf: Vec<u8>) {}

fn arithmetic(buf: Vec<u8>, len: usize) {}

fn block_rm(buf: Vec<u8>) {}

fn block_insert(buf: Vec<u8>) {}

fn block_swap(buf: Vec<u8>) {}

fn block_shuffle(buf: Vec<u8>) {}

fn block_merge(buf: Vec<u8>) {}
