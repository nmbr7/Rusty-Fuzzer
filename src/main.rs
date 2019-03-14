#![allow(unused)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;
mod config;
mod fuzzstat;
use config::{ProgConfig, SeedConfig};
use fuzzstat::{FuzzerStatus};
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
//use scheduler;
//use mutengine;
//use execengine;
//use configupdater;

fn main() {
    let args = clap_app!(fuzzer =>
    (version: env!("CARGO_PKG_VERSION"))
    (author: env!("CARGO_PKG_AUTHORS"))
    (about: "A grey box evolutionary fuzzer")
    (@arg SEED_FILE_DIR: -s --seed +takes_value +required "Seed file to use")
    (@arg INPUT_DIR: -i --input +takes_value  "Input file name")
    (@arg OUTPUT: -o --output +takes_value "Output directory Name")
    (@arg TIMEOUT: -l --limit +takes_value "Execution timeout limit")
    )
    .get_matches();

    let input = args.value_of("INPUT_DIR").unwrap();
    let output = args.value_of("OUTPUT_DIR").unwrap_or(input);
    let timeout = args.value_of("TIMEOUT").unwrap_or("30");
    let seed = args.value_of("SEED_FILE_DIR").unwrap();
    let mut conf_queue = SeedConfig::init_queue(seed).unwrap();
    let mut fuzzer_status = FuzzerStatus::init(conf_queue.len());

    /// Debug
    for i in 0..conf_queue.len() {
        println!(
            " \nseed : {:?}\ninput : {}\nOutDir : {}\nTimeout : {}\n",
            conf_queue[i], input, output, timeout
        );
    fuzzer_status.update(conf_queue.len());
    }
    /// Debug
    println!("{:?}\n", fuzzer_status);
    let prog_config = ProgConfig::init(
        input.to_string(),
        output.to_string(),
        timeout.parse::<u32>().unwrap(),
    );
}
