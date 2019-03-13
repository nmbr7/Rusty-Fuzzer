#![allow(unused)]
#[macro_use]
extern crate clap;
mod config;
use config::{SeedConfig, ProgConfig,FuzzerStatus};
use std::io::{self,BufReader};
use std::io::prelude::*;
use std::fs::File;

//use scheduler;
//use mutengine;
//use execengine;
//use configupdater;
//use procstat;

fn main() {
    let args = clap_app!(fuzzer =>
    (version: env!("CARGO_PKG_VERSION"))
    (author: env!("CARGO_PKG_AUTHORS"))
    (about: "A grey box evolutionary fuzzer")
    (@arg SEED_FILE: -s --seed +takes_value +required "Seed file to use")
    (@arg INPUT: -i --input +takes_value +required "Input file name")
    (@arg OUTPUT: -o --output +takes_value "Output directory Name")
    (@arg TIMEOUT: -l --limit +takes_value "Execution timeout limit")
    )
    .get_matches();


    let input = args.value_of("INPUT").unwrap();
    let mut default_out_dir = input.to_string();
    default_out_dir.push_str("_FuzzDir");
    let output = args.value_of("OUTPUT").unwrap_or(default_out_dir.as_str());
    let timeout = args.value_of("TIMEOUT").unwrap_or("5");
    let seed = args.value_of("SEED_FILE").unwrap();
    
    let file = File::open(seed).expect("Failed to open seed file");
    let readbuf = BufReader::new(file);

    let mut seedv: Vec<String> = Vec::new();
    for seed in readbuf.lines(){
        seedv.push(seed.unwrap());
    }

    println!("seed : {:?}\ninput : {}\nOutDir : {}\nTimeout : {}",seedv,input,output,timeout);

    let fuzzer_status = FuzzerStatus::init(Some(seedv.len()));
    let prog_config = ProgConfig::init(input.to_string(),output.to_string(),timeout.parse::<u8>().unwrap());
    let seed_config = SeedConfig::new(Some(seedv));
}
