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
    BlockShfl,
    Reverse,
    None,
}

pub fn mutate(
    seed_queue: &mut VecDeque<SeedConfig>,
    rand: usize,
    fuzzstatus: &mut FuzzerStatus,
) -> SeedConfig {
    let mut buf = seed_queue[rand].seed.clone();
    let mut len = seed_queue[rand].newlen;
    let fname = seed_queue[rand].seedfile.clone();
    /*unsafe{
    println!("{}",String::from_utf8_unchecked(buf.clone()));

    }println!(" {} {}",len,buf.len());
       */
    if len != 0 || buf.len() == len {
        len -= 2;
    }
    let (buf, mutant) = loop {
        let (mut buf, mutant) = match random(8) {
            1 => ascii_mod(len, buf.clone()),
            0 => bit_flip(len, buf.clone()),
            2 => nibble_flip(len, buf.clone()),
            3 => block_insert(len, buf.clone()),
            4 => byte_mod(len, buf.clone()),
            5 => hot_values(len, buf.clone()),
            6 => block_shuffle(len, buf.clone()),
            7 => block_rm(len, buf.clone()),
            _ => panic!("Unknown"),
        };

        //buf.retain(|&x| x.is_ascii_alphanumeric());
        //        buf.retain(|&x| x.is_ascii_digit());

        //      if buf.bytes().all(|b| b.unwrap().is_as`cii_alphanumeric()) {
        if !buf.is_empty() {
            break (buf, mutant);
        }
    };

    //    println!("{:?}",buf);
    let mut seed = SeedConfig::new(
        format!("{}_{}", fname, fuzzstatus.time_elapsed.as_nanos()),
        buf,
        fuzzstatus.conf_count + 1,
        seed_queue[rand].gen + 1,
        seed_queue[rand].newlen,
    );
    seed.mutation = Mutation {
        parent: seed_queue[rand].id,
        mutant,
    };
    //seed.fitness = seed_queue[rand].fitness;
    seed.fitness = 0;
    seed
}

fn bit_flip(len: usize, mut buf: Vec<u8>) -> (Vec<u8>, MutType) {
    let mut buf = buf;
    let pos = if len < buf.len() {
        random_range(len, buf.len())
    } else {
        random_range(0, buf.len())
    };
    buf[pos] = match random(3) {
        //Optimize
        0 => buf[pos] ^ (1 << (random(pos * 8 + 1) % 8)),
        1 => buf[pos] ^ (3 << (random(pos * 8 + 1) % 7)),
        2 => buf[pos] ^ (7 << (random(pos * 8 + 1) % 6)),
        _ => panic!("Unknown"),
    };
    (buf, MutType::BitFlip)
}

fn nibble_flip(len: usize, mut buf: Vec<u8>) -> (Vec<u8>, MutType) {
    let pos = if len < buf.len() {
        random_range(len, buf.len())
    } else {
        random_range(0, buf.len())
    };
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

fn byte_mod(len: usize, mut buf: Vec<u8>) -> (Vec<u8>, MutType) {
    let pos = if len < buf.len() {
        random_range(len, buf.len())
    } else {
        random_range(0, buf.len())
    };
    buf[pos] = random(128) as u8;
    //A bit more
    (buf, MutType::ByteMod)
}

fn int_mod(len: usize, buf: Vec<u8>) -> (Vec<u8>, MutType) {
    let pos = if len < buf.len() {
        random_range(len, buf.len())
    } else {
        random_range(0, buf.len())
    };
    //buf[pos] = ;

    (buf, MutType::IntMod)
}

fn ascii_mod(len: usize, mut buf: Vec<u8>) -> (Vec<u8>, MutType) {
    let pos = if len < buf.len() {
        random_range(len, buf.len())
    } else {
        random_range(0, buf.len())
    };
    buf[pos] = match random(3) {
        0 => random_range(0x41, 0x5a) as u8,
        1 => random_range(0x61, 0x7a) as u8,
        2 => random_range(0x30, 0x39) as u8,
        _ => panic!("Unknown"),
    };

    (buf, MutType::AsciiMod)
}

fn hot_values(len: usize, mut buf: Vec<u8>) -> (Vec<u8>, MutType) {
    let pos = if len < buf.len() {
        random_range(len, buf.len())
    } else {
        random_range(0, buf.len())
    };
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

fn block_insert(len: usize, mut buf: Vec<u8>) -> (Vec<u8>, MutType) {
    let pos = if len < buf.len() {
        random_range(len, buf.len())
    } else {
        random_range(0, buf.len())
    };
    for _ in 0..=random(4) {
        buf.push(random_range(20, 128) as u8);
    }
    if random(3) % 2 == 0 {
        let mut p = &mut buf[..];
        p.reverse();
        buf = p.to_vec();
    }
    (buf, MutType::BlockInsert)
}

fn block_rm(len: usize, mut buf: Vec<u8>) -> (Vec<u8>, MutType) {
    let pos = if len < buf.len() {
        random_range(len, buf.len())
    } else {
        random_range(0, buf.len())
    };
    buf.remove(pos);
    if random(3) % 2 == 0 {
        let mut p = &mut buf[..];
        p.reverse();
        buf = p.to_vec();
    }
    (buf, MutType::BlockRm)
}

fn block_shuffle(len: usize, mut buf: Vec<u8>) -> (Vec<u8>, MutType) {
    for _ in 0..=random(2) {
        let ins = random(buf.len());
        let rmv = random(buf.len());
        let temp = buf.remove(rmv);
        buf.insert(ins, temp);
        if random(3) % 2 == 0 {
            let mut p = &mut buf[..];
            p.reverse();
            buf = p.to_vec();
        }
    }
    (buf, MutType::BlockShfl)
}

fn arithmetic(buf: Vec<u8>, len: usize) {}
fn block_swap(buf: Vec<u8>) {}
fn block_merge(buf: Vec<u8>) {}
