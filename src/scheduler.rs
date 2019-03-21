use rand;
use rand::distributions::{Distribution, Uniform};
use crate::config::{ProgConfig, SeedConfig};
use std::collections::VecDeque;
use crate::execengine::exec_fuzz;

pub fn sched(seed_queue : &mut VecDeque<SeedConfig>,progconfig : ProgConfig) {
    
    for _ in 0..1000{
         
        let mut range = Uniform::from(0..seed_queue.len()+1);
    let mut rng = rand::thread_rng();
    let mut random = range.sample(&mut rng);
    exec_fuzz(&mut seed_queue[random],&progconfig);
    println!("updated seed {:?}",&seed_queue);

    }

}
