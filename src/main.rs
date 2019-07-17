#![allow(unused)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;

mod config;
mod confupdater;
mod execengine;
mod fuzzstat;
mod helpertools;
mod mutengine;
mod scheduler;
use config::{ProgConfig, SeedConfig};
use fuzzstat::FuzzerStatus;
use scheduler::sched;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() {
    let args = clap_app!(rusty_fuzzer =>
    (version: env!("CARGO_PKG_VERSION"))
    (author: env!("CARGO_PKG_AUTHORS"))
    (about: "A grey box evolutionary fuzzer")
    (@arg SEED_FILE_DIR: -s --seed +takes_value +required "Seed directory to use")
    (@arg INPUT_COMMAND: -i --input +takes_value +required "Input program and arguments where the argument to be fuzzed is specified by '@'")
    //(@arg TIMEOUT: -l --limit +takes_value "Execution timeout limit")
    (@arg INPUT_TYPE: -t --inputtype +takes_value "Input type taken by the program (FileInput(f) or command line TextInput(c))")
    )
    .get_matches();

    let inputcommand = args.value_of("INPUT_COMMAND").unwrap();
    let timeout = args.value_of("TIMEOUT").unwrap_or("30");
    let seedfile_dir = args.value_of("SEED_FILE_DIR").unwrap();
    let intype = args.value_of("INPUT_TYPE").unwrap();

    let prog_config = ProgConfig::init(
        inputcommand.trim_start().to_string(),
        timeout.parse::<u32>().unwrap(),
        intype.to_string(),
    );
    let mut seed_queue =
        SeedConfig::init_queue(seedfile_dir, prog_config.prog_name.clone(), intype).unwrap();
    let mut fuzzer_status = FuzzerStatus::init(seed_queue.len());
    sched(&mut seed_queue, prog_config, &mut fuzzer_status);

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
