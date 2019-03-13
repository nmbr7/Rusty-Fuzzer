extern crate chrono;
use chrono::{DateTime, Utc};

use std::collections::hash_map::DefaultHasher;
use std::time::{Duration, Instant, SystemTime};

pub struct SeedConfig {
    id: Option<u32>,
    seed: Option<Vec<String>>,
    time: Option<ExecTime>,
    parents: Option<Vec<u64>>,
    exit_stat: Option<Stat>,
    fitness: Option<u8>,
}

impl SeedConfig {
    pub fn new(seed: Option<Vec<String>>) -> SeedConfig {
        SeedConfig {
            seed,
            time: Some(ExecTime {
                limit: None,
                total: None,
            }),
            id: Some(0),
            exit_stat: None,
            fitness: None,
            parents: None,
        }
    }
}

struct ExecTime {
    limit: Option<u8>,
    total: Option<Duration>,
}

struct CrashHash {
    headhash: DefaultHasher,
    tailhash: DefaultHasher,
    fullhash: DefaultHasher,
}

enum Stat {
    SUCCESS,
    CRASH(CrashHash),
    HANG,
}
pub struct ProgConfig {
    inputpath:  String,
    outputdir:  String,
    timeout: u8,
}

impl ProgConfig {
    pub fn init(inputpath:  String , outputdir: String , limit: u8) -> ProgConfig {
        ProgConfig {
            inputpath,
            outputdir,
            timeout: limit,
        }
    }
}

pub struct FuzzerStatus {
    start_time: (DateTime<Utc>, Instant),
    crash_count: Option<u32>,
    test_count: Option<u32>,
    conf_count: Option<usize>,
    queue_len: Option<u32>,
    valid_crashes: Option<u32>,
}

impl FuzzerStatus {
    pub fn init(conf_count: Option<usize>) -> FuzzerStatus {
        FuzzerStatus {
            conf_count,
            start_time: (
                Utc::now(), /*.format("%a %b %e %T %Y")*/
                Instant::now(),
            ),
            crash_count: None,
            queue_len: None,
            valid_crashes: None,
            test_count: None,
        }
    }
}
