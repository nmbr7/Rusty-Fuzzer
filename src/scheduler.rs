use crate::config::{ProgConfig, SeedConfig, Stat};
use crate::execengine::exec_fuzz;
use crate::fuzzstat::FuzzerStatus;
use crate::helpertools::random;
use crate::mutengine::mutate;
use std::collections::VecDeque;

pub fn sched(
    seed_queue: &mut VecDeque<SeedConfig>,
    progconfig: ProgConfig,
    fuzzer_status: &mut FuzzerStatus,
) {
    for i in 0..100000 {
        let rand = random(seed_queue.len());

        seed_queue.push_back(mutate(&seed_queue[rand], &seed_queue, fuzzer_status));
        fuzzer_status.newseed(seed_queue.len());
        //        for i in 0..seed_queue.len() {
        //          println!(" \nseed : {:?}\n", seed_queue[i]);
        //    }

        //Proper scheduling
        let rand = random(seed_queue.len());
        exec_fuzz(&mut seed_queue[rand], &progconfig);
        fuzzer_status.update(seed_queue.len(), &seed_queue[rand].exit_stat);

        if (i % 5000 == 0) {
            println!(
                "\n\n
                 -- Fuzzer Status --\n
                 Start Time     : {:?}\n
                 Time Elapsed   : {:?}\n
                 Queue Length   : {}\n
                 Crash Count    : {}\n
                 Configs Tested : {}",
                &fuzzer_status.start_time.0,
                &fuzzer_status.time_elapsed,
                &fuzzer_status.queue_len,
                &fuzzer_status.crash_count,
                &fuzzer_status.conf_count
            );
        }
    }
}
