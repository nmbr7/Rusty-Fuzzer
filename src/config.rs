extern crate chrono;
use chrono::{DateTime, Utc};

use std::time::{Duration, Instant, SystemTime};
use std::collections::hash_map::DefaultHasher;

pub struct SeedConf {
    id: u32,
    seed: Vec<u64>,
    time: ExecTime,
    parents: Vec<u64>,
    exit_stat: Stat,
    fitness: u8,   
}

impl SeedConf{
    pub fn new(seed: Vec<u64>, limit: u8,id: u32 ) -> SeedConf {
        SeedConf{
            seed,
            time: ExecTime {limit},
            id,
        }
    }
}

struct ExecTime {
    limit: u8,
    total: Duration,
}

struct CrashHash{
    headhash: DefaultHasher,
    tailhash: DefaultHasher,
    fullhash: DefaultHasher,
}

enum Stat{
    SUCCESS,
    CRASH(CrashHash),
    HANG,
}
pub struct ProgConf {
    inputpath: String,
    outputdir: String,
}

impl ProgConf{
    pub fn init(inputpath:String, outputdir:String) -> ProgConf {
        ProgConf{
            inputpath,
            outputdir,
        }
    }
} 

pub struct FuzzerStatus {
    start_time: (DateTime<Utc>,Instant),
    crash_count: u32,
    test_count: u32,
    conf_count: u32,
    queue_len: u32,
    valid_crashes: u32,
}

impl FuzzerStatus{
    pub fn init(conf_count: u32) -> FuzzerStatus {
        FuzzerStatus{
            conf_count,
            start_time: (Utc::now()/*.format("%a %b %e %T %Y")*/,Instant::now()),
        }
    }
} 
