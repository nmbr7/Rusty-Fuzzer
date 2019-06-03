use crate::config::{ProgConfig, SeedConfig, Stat};
use crate::confupdater::conf_update;
use crate::execengine::exec_fuzz;
use crate::fuzzstat::FuzzerStatus;
use crate::helpertools::{random, random_range};
use crate::mutengine::mutate;
use std::collections::VecDeque;

pub fn sched(
    seed_queue: &mut VecDeque<SeedConfig>,
    progconfig: ProgConfig,
    fuzzer_status: &mut FuzzerStatus,
) {
    let mut g = 0;
    let mut i = 0;
    loop {
        //for i in 0..10000{
        let rand = random(seed_queue.len());
        seed_queue[rand].evolved += 1;

        //mutate the seed
        let mut_seed = mutate(seed_queue, rand, fuzzer_status);
        seed_queue.retain(|x| x.seed != mut_seed.seed);
        seed_queue.push_back(mut_seed);

        //update queue length in fuzzer status struct
        fuzzer_status.newseed(seed_queue.len());

        //Proper scheduling

        let rand = random(seed_queue.len());

        //execute the test target
        exec_fuzz(&mut seed_queue[rand], &progconfig, fuzzer_status);

        //update the fuzzer status
        fuzzer_status.update(seed_queue.len(), &seed_queue[rand].exit_stat);

        //update config queue
        conf_update(seed_queue, fuzzer_status, &i, &mut g);

        // Fuzzer status output
     //   if (i % 50 == 0) {
            println!(
                "\n\n
                 -- Fuzzer Status --\n
                 Start Time     : {:?}\n
                 Time Elapsed   : {:?}\n
                 Queue Length   : {}\n
                 Crash Count    : {}\n
                 Configs Tested : {}\n
                 Coverage Count : {}",
                &fuzzer_status.start_time.0,
                &fuzzer_status.time_elapsed,
                &fuzzer_status.queue_len,
                &fuzzer_status.crash_count,
                &fuzzer_status.conf_count,
                &fuzzer_status.coverage_count.0,
            );
       // }

        i += 1;
    }
}
