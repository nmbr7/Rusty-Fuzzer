extern crate chrono;
use crate::mutengine::{MutType, Mutation};
use chrono::{DateTime, Utc};
use std::collections::hash_map::DefaultHasher;
use std::collections::VecDeque;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::time::{Duration, Instant, SystemTime};

#[derive(Debug, Clone)]
pub struct SeedConfig {
    pub arg_count: usize,
    pub id: usize,
    pub seed: Vec<u8>,
    pub time: ExecTime,
    pub mutation: Mutation,
    pub exit_stat: Stat,
    pub fitness: u8,
    pub output: String,
    pub input: String,
}

impl SeedConfig {
    pub fn new(seed: Vec<u8>, id: usize) -> Self {
        Self {
            arg_count: seed.len(),
            seed,
            time: ExecTime {
                limit: 0,
                total: [].to_vec(),
            },
            id,
            mutation: Mutation {
                parent: id,
                mutant: MutType::None,
            },
            exit_stat: Stat::NONE,
            fitness: 0,
            output: format!("crash_{}", id),
            input: format!("seed_{}", id),
        }
    }

    pub fn update() -> std::io::Result<()> {
        Ok(())
    }

    pub fn init_queue(seedfile: &str, input: String) -> std::io::Result<VecDeque<SeedConfig>> {
        let mut config_queue: VecDeque<SeedConfig> = VecDeque::new();
        for (id, path) in fs::read_dir(&seedfile)?.enumerate() {
            let file = path.unwrap().path();
            println!("{:?}", file);
            let mut f = File::open(file)?;
            let mut buf = Vec::new();
            //let mut seedv: Vec<String> = Vec::new();
            f.read_to_end(&mut buf)?;
            //for line in f.lines() {
            //  seedv.push(line.unwrap());
            // }
            let conf = SeedConfig::new(buf, id);
            File::create(format!("{}_FuzzDir/input_set/{}", input, conf.input)).unwrap();

            config_queue.push_back(conf);
        }
        Ok(config_queue)
    }
}
#[derive(Debug, Clone)]
pub struct ExecTime {
    pub limit: u32,
    pub total: Vec<Duration>,
}
/*#[derive(Debug, Clone)]
pub struct CrashHash {
    pub headhash: DefaultHasher,
    pub tailhash: DefaultHasher,
    pub fullhash: DefaultHasher,
}
*/
#[derive(Debug, Clone)]
pub enum Stat {
    NONE,
    SUCCESS,
    CRASH, //(CrashHash),
    HANG,
}
#[derive(Debug, Clone)]
pub struct ProgConfig {
    pub inputpath: String,
    pub outputdir: String,
    pub timeout: u32,
}

impl ProgConfig {
    pub fn init(inputpath: String, limit: u32) -> Self {
        fs::create_dir_all(format!("{}_FuzzDir/Crash", inputpath)).unwrap();
        fs::create_dir_all(format!("{}_FuzzDir/input_set", inputpath)).unwrap();
        File::create(format!("{}_FuzzDir/log", inputpath)).unwrap();
        Self {
            inputpath: inputpath.clone(),
            outputdir: format!("{}_FuzzDir", inputpath),
            timeout: limit,
        }
    }
}
