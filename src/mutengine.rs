use crate::config::{ProgConfig, SeedConfig, Stat};
use crate::helpertools::random;
use std::collections::VecDeque;

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

pub fn mutate(seed_queue: &mut VecDeque<SeedConfig>) -> () {
    ()
}

fn bit_flip(buf: Vec<u8>) {}

fn nibble_mod(buf: Vec<u8>, count: usize) {}

fn int_mod(buf: Vec<u8>) {}

fn ascii_mod(buf: Vec<u8>) {}

fn hot_values(buf: Vec<u8>) {}

fn arithmetic(buf: Vec<u8>, len: usize) {}

fn block_rm(buf: Vec<u8>) {}

fn block_insert(buf: Vec<u8>) {}

fn block_swap(buf: Vec<u8>) {}

fn block_shuffle(buf: Vec<u8>) {}

fn block_merge(buf: Vec<u8>) {}
