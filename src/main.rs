#![allow(unused)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;

mod config;
mod execengine;
mod fuzzstat;
mod scheduler;
mod mutengine;
mod helpertools;
//mod configupdater;
use config::{ProgConfig, SeedConfig};
use fuzzstat::FuzzerStatus;
use scheduler::sched;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() {
    let args = clap_app!(fuzzer =>
    (version: env!("CARGO_PKG_VERSION"))
    (author: env!("CARGO_PKG_AUTHORS"))
    (about: "A grey box evolutionary fuzzer")
    (@arg SEED_FILE_DIR: -s --seed +takes_value +required "Seed file to use")
    (@arg INPUT_DIR: -i --input +takes_value  "Input file name")
    (@arg TIMEOUT: -l --limit +takes_value "Execution timeout limit")
    )
    .get_matches();

    let input = args.value_of("INPUT_DIR").unwrap();
    let timeout = args.value_of("TIMEOUT").unwrap_or("30");
    let seed = args.value_of("SEED_FILE_DIR").unwrap();
    let prog_config = ProgConfig::init(input.to_string(), timeout.parse::<u32>().unwrap());
    let mut conf_queue = SeedConfig::init_queue(seed, prog_config.inputpath.clone()).unwrap();
    let mut fuzzer_status = FuzzerStatus::init(conf_queue.len());
    sched(&mut conf_queue, prog_config, &mut fuzzer_status);

    /* Debug
    for i in 0..conf_queue.len() {
        println!(
            " \nseed : {:?}\ninput : {}\nTimeout : {}\n",
            conf_queue[i], input, timeout
        );
    //fuzzer_status.update(conf_queue.len());
    }
    */

    /*for i in 0..conf_queue.len() {
    println!(
        " \nseed : {:?}\ninput : {}\nTimeout : {}\n",
        conf_queue[i], input, timeout
    );
    }*/
}
