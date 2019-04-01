use crate::config::{ProgConfig, SeedConfig, Stat};
use crate::fuzzstat::FuzzerStatus;
use crate::helpertools::{random, random_range};
use std::collections::VecDeque;
use std::format;
use std::io::Read;
#[derive(Debug, Clone)]
pub struct Mutation {
    pub parent: usize,
    pub mutant: MutType,
}

#[derive(Debug, Clone)]
pub enum MutType {
    BitFlip,
    NibbleFlip,
    ByteMod,
    IntMod,
    AsciiMod,
    HotValues,
    ArithMetic,
    BlockRm,
    BlockInsert,
    BlockSwap,
    None,
}

pub fn mutate(
    seed_config: &SeedConfig,
    seed_queue: &VecDeque<SeedConfig>,
    fuzzstatus: &mut FuzzerStatus,
) -> SeedConfig {
    let mut buf = seed_config.seed.clone();
    let (buf, mutant) = loop {
        let (mut buf, mutant) = match random(5) {
            0 => bit_flip(buf.clone()),
            1 => nibble_flip(buf.clone()),
            2 => byte_mod(buf.clone()),
            3 => hot_values(buf.clone()),
            4 => block_insert(buf.clone()),
            5 => ascii_mod(buf.clone()),
            _ => panic!("Unknown"),
        };

        buf.retain(|&x| x.is_ascii_alphanumeric());

        //      if buf.bytes().all(|b| b.unwrap().is_ascii_alphanumeric()) {
        if !buf.is_empty() {
            break (buf, mutant);
        }
    };

    //    println!("{:?}",buf);
    let mut seed = SeedConfig::new(buf, fuzzstatus.conf_count + 1, seed_config.gen + 1);
    seed.mutation = Mutation {
        parent: seed_config.id,
        mutant,
    };
    seed.fitness = seed_config.fitness;
    seed
}

fn bit_flip(mut buf: Vec<u8>) -> (Vec<u8>, MutType) {
    let mut buf = buf;
    let pos = random(buf.len());
    buf[pos] = match random(3) {
        //Optimize
        0 => buf[pos] ^ (1 << (random(pos * 8 + 1) % 8)),
        1 => buf[pos] ^ (3 << (random(pos * 8 + 1) % 7)),
        2 => buf[pos] ^ (7 << (random(pos * 8 + 1) % 6)),
        _ => panic!("Unknown"),
    };
    (buf, MutType::BitFlip)
}

fn nibble_flip(mut buf: Vec<u8>) -> (Vec<u8>, MutType) {
    let pos = random(buf.len());
    buf[pos] = match random(7) {
        0 | 2 | 4 | 6 => buf[pos] ^ (0xf << (random(pos * 8 + 1) % 4)),
        1 | 3 | 5 => buf[pos] ^ 0xff,
        _ => panic!("Unknown"),
    };

    /**7 | 8 => {
        buf[pos] ^ 0b11111111,
        buf[pos+1]^0b11111111
    }**/
    (buf, MutType::NibbleFlip)
}

fn byte_mod(mut buf: Vec<u8>) -> (Vec<u8>, MutType) {
    let pos = random(buf.len());
    buf[pos] = random(128) as u8;
    //A bit more
    (buf, MutType::ByteMod)
}

fn int_mod(buf: Vec<u8>) -> (Vec<u8>, MutType) {
    let pos = random(buf.len());
    //buf[pos] = ;

    (buf, MutType::IntMod)
}

fn ascii_mod(mut buf: Vec<u8>) -> (Vec<u8>, MutType) {
    let pos = random(buf.len());
    buf[pos] = match random(3) {
        0 => random_range(0x41, 0x5a) as u8,
        1 => random_range(0x61, 0x7a) as u8,
        2 => random_range(0x30, 0x39) as u8,
        _ => panic!("Unknown"),
    };

    (buf, MutType::AsciiMod)
}
fn hot_values(mut buf: Vec<u8>) -> (Vec<u8>, MutType) {
    let pos = random(buf.len());
    match random(2) {
        0 => buf[pos] = 255 as u8,
        1 => buf[pos] = 0 as u8,
        _ => panic!("Unknown"),
    };
    /**2 => ,
    3 => ,
    4 => ,
    5 => ,
    6 => ,
    7 => ,
    8 => ,
    9 => ,**/
    (buf, MutType::HotValues)
}

fn arithmetic(buf: Vec<u8>, len: usize) {}

fn block_insert(mut buf: Vec<u8>) -> (Vec<u8>, MutType) {
    let pos = random(buf.len());
    for _ in 0..=random(4) {
        buf.push(random(128) as u8);
    }
    /**    match [1, 2, 3, 4][random(1)] {
        1 => buf.push(random(128) as u8),

    2 => {**/
    /**},
                                                                     3 => ,
    4 => ,
        _ => panic!("Unknown"),
    };**/
    (buf, MutType::BlockInsert)
}
fn block_rm(buf: Vec<u8>) {}

fn block_swap(buf: Vec<u8>) {}

fn block_shuffle(buf: Vec<u8>) {}

fn block_merge(buf: Vec<u8>) {}
