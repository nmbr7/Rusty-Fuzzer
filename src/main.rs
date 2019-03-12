#![allow(unused)]
#[macro_use]
extern crate clap;
mod config;
use config::{SeedConf, ProgConf,FuzzerStatus};

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
    (@arg INPUT: -i --input +takes_value "Input file name")
    (@arg OUTPUT: -o --output +takes_value "Output directory Name")
    (@arg EXEC_LIMIT: -l --limit +takes_value "Execution timeout limit")
    )
    .get_matches();
}
