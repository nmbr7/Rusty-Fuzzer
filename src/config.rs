extern crate chrono;
use crate::fuzzstat::FuzzerStatus;
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
    pub seed: Vec<u8>,
    pub seed_len: usize,
    pub id: usize,
    pub seedfile: String,
    pub time: ExecTime,
    pub mutation: Mutation,
    pub evolved: usize,
    pub gen: usize,
    pub exit_stat: Stat,
    pub fitness: u8,
    pub newlen: usize,
    pub outputf: String,
    pub inputf: String,
}

impl SeedConfig {
    pub fn new(seedfile: String, seed: Vec<u8>, id: usize, gen: usize, nl: usize) -> Self {
        Self {
            seed_len: seed.len(),
            seed,
            seedfile,
            time: ExecTime {
                limit: 0,
                total: [].to_vec(),
            },
            id,
            gen,
            newlen: nl,
            evolved: 0,
            mutation: Mutation {
                parent: id,
                mutant: MutType::None,
            },
            exit_stat: Stat::NONE,
            fitness: 0,
            outputf: format!("crash_{}", id),
            inputf: format!("seed_{}", id),
        }
    }

    pub fn update() -> std::io::Result<()> {
        Ok(())
    }

    pub fn init_queue(
        seedfile: &str,
        prog: String,
        intype: &str,
    ) -> std::io::Result<VecDeque<SeedConfig>> {
        let mut seed_queue: VecDeque<SeedConfig> = VecDeque::new();

        for (id, path) in fs::read_dir(&seedfile)?.enumerate() {
            let file = path.unwrap().path();
            println!("Seed file name :{:?}", file);
            let mut f = File::open(&file)?;
            let mut buf = Vec::new();
            f.read_to_end(&mut buf)?;
            let conf = SeedConfig::new(file.to_str().unwrap().to_string(), buf.clone(), id, 0, 0);

            let mut writefile =
                File::create(format!("{}_FuzzDir/input_set/{}", prog, conf.inputf)).unwrap();
            unsafe {
                writefile.write_all(String::from_utf8_unchecked(buf).as_bytes());
            }
            seed_queue.push_back(conf);
            //println!("First seed {}",String::from_utf8_unchecked(seed_config.seed.clone()));
            //FuzzerStatus::newseed(seed_queue.len());
        }
        Ok(seed_queue)
    }

    pub fn new_seed_file(&self, prog: String) {
        let mut writefile = File::create(format!("{}_FuzzDir/current_seed", prog)).unwrap();
        unsafe {
            writefile.write_all(String::from_utf8_unchecked(self.seed.clone()).as_bytes());
        }
    }

    pub fn seed_queue_update(&self, seedq: &mut VecDeque<SeedConfig>, prog: String) {
        seedq.push_back(self.clone());
        let mut writefile =
            File::create(format!("{}_FuzzDir/input_set/{}", prog, &self.inputf)).unwrap();
        unsafe {
            writefile.write_all(String::from_utf8_unchecked(self.seed.clone()).as_bytes());
        }
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
    pub prog_name: String,
    pub prog_args: String,
    pub outputdir: String,
    pub inputtype: String,
    pub timeout: u32,
}

impl ProgConfig {
    pub fn init(inputcommand: String, limit: u32, inputtype: String) -> Self {
        let prog_name = &inputcommand.splitn(2, ' ').collect::<Vec<&str>>()[0].to_string();
        fs::create_dir_all(format!("{}_FuzzDir/Crash", prog_name)).unwrap();
        fs::create_dir_all(format!("{}_FuzzDir/input_set", prog_name)).unwrap();
        File::create(format!("{}_FuzzDir/log", prog_name)).unwrap();
        File::create(format!("{}_FuzzDir/current_seed", prog_name)).unwrap();
        Self {
            prog_name: prog_name.clone(),
            prog_args: inputcommand,
            outputdir: format!("{}_FuzzDir", prog_name),
            timeout: limit,
            inputtype: inputtype.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::env;
    use std::path::Path;

    fn test_init() {
        let cr = env::current_dir().unwrap();
        let cdir = format!("{}/test_output_dir/seed", cr.display());
        println!("Current directory is {:?}", cr);
        fs::create_dir_all(&cdir);
        let cdir = format!("{}/test_output_dir", cr.display());
        assert!(env::set_current_dir(&cdir).is_ok());
        let cr = env::current_dir().unwrap();
        println!("Current directory is {:?}", cr);
    }

    fn test_cleanup() {
        let cr = env::current_dir().unwrap();
        println!("Current directory is {:?}", cr);
        fs::remove_dir_all(format!("{}/test_output_dir", cr.display()));
    }

    #[test]
    fn test_prog_and_seed_config() {
        test_init();
        let prog_config =
            ProgConfig::init("ls -la -p -q -w @ name".to_string(), 10, "f".to_string());
        println!("\n{:?}\n", prog_config);
        let mut seed_queue = SeedConfig::init_queue(
            &"seed".to_string(),
            prog_config.prog_name.clone(),
            &prog_config.inputtype.clone(),
        )
        .unwrap();
        println!("\n{:?}\n", seed_queue);
    }

}
