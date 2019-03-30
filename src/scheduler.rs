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
    for _ in 0..10 {
        let rand = random(seed_queue.len());

        let new_seed = mutate(seed_queue);

        exec_fuzz(&mut seed_queue[rand], &progconfig);
        fuzzer_status.update(seed_queue.len(), &seed_queue[rand].exit_stat);

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
