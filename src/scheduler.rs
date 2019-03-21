use rand;
use rand::distributions::{Distribution, Uniform};
use crate::config::{ProgConfig, SeedConfig,Stat};
use std::collections::VecDeque;
use crate::execengine::exec_fuzz;
use crate::fuzzstat::{FuzzerStatus};

pub fn sched(seed_queue : &mut VecDeque<SeedConfig>,progconfig : ProgConfig,fuzzer_status : &mut FuzzerStatus) {
    
   for _ in 0..4{
         
    let mut range = Uniform::from(0..seed_queue.len());
    let mut rng = rand::thread_rng();
    let mut random = range.sample(&mut rng);
    exec_fuzz(&mut seed_queue[random],&progconfig);
    fuzzer_status.update(seed_queue.len(),&seed_queue[random].exit_stat);
    println!("Updated_seed {:?}\n\n Fuzzer_status {:?}\n\n",&seed_queue[random],&fuzzer_status);

    }

}
