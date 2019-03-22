use crate::config::{ProgConfig, SeedConfig, Stat};
use crate::execengine::exec_fuzz;
use crate::fuzzstat::FuzzerStatus;
use rand;
use rand::distributions::{Distribution, Uniform};
use std::collections::VecDeque;

pub fn sched(
    seed_queue: &mut VecDeque<SeedConfig>,
    progconfig: ProgConfig,
    fuzzer_status: &mut FuzzerStatus,
) {
    for _ in 0..100 {
        let mut range = Uniform::from(0..seed_queue.len());
        let mut rng = rand::thread_rng();
        let mut random = range.sample(&mut rng);
        exec_fuzz(&mut seed_queue[random], &progconfig);
        fuzzer_status.update(seed_queue.len(), &seed_queue[random].exit_stat);
        println!("\n\t\t\t\t-- Fuzzer Status --\n Start Time : {:?}\tTime Elapsed : {:?}\tQueue Length : {}\t Crash Count : {}",&fuzzer_status.start_time,&fuzzer_status.time_elapsed,&fuzzer_status.queue_len,&fuzzer_status.crash_count);
        //println!("Fuzzer_status\n {:?}\n\n",&fuzzer_status);
    }
}
