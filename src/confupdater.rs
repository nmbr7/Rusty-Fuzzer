use crate::config::{ProgConfig, SeedConfig, Stat};
use crate::fuzzstat::FuzzerStatus;
use crate::helpertools::{random, random_range};
use std::collections::VecDeque;

pub fn conf_update(
    seed_queue: &mut VecDeque<SeedConfig>,
    fuzzer_status: &mut FuzzerStatus,
    iter: usize,
    gen: &mut usize,
) {
    //seed_queue.retain(|x| !mut_seed.seed.starts_with(&x.seed) && x.seed.len() > random_range(3, 10));
    if ((iter + 1) % 101 == 0) {
        seed_queue.retain(|x| x.evolved < 6 && x.fitness >= (fuzzer_status.coverage_count.0 - 1));
        //seed_queue.remove(rand);
    }
    //     println!("{}",seed_queue.len());

    if ((iter + 1) % 1001 == 0) {
        //    seed_queue.retain(|x| x.fitness  ));
        seed_queue
            .retain(|x| /*x.gen > *gen &&*/ x.fitness >= (fuzzer_status.coverage_count.0 - 1));
        *gen += 1;
    }
}
