use crate::config::{Stat};
use std::time::{Duration, Instant, SystemTime};
use chrono::{DateTime, Utc};
#[derive(Debug, Clone)]
pub struct FuzzerStatus {
    start_time: (DateTime<Utc>, Instant),
    time_elapsed: Duration,
    crash_count: u32,
    test_count: u32,
    conf_count: usize,
    queue_len: usize,
    valid_crashes: u32,
}

impl FuzzerStatus {
    pub fn init(conf_count: usize) -> Self {
        Self {
            conf_count,
            start_time: (
                Utc::now(), /*.format("%a %b %e %T %Y")*/
                Instant::now(),
            ),
            time_elapsed: Duration::from_secs(0),
            crash_count: 0,
            queue_len: conf_count,
            valid_crashes: 0,
            test_count: 0,
        }
    }

    pub fn update(&mut self, quelen: usize,exit_stat : &Stat) -> bool {
        self.crash_count = match exit_stat {
            Stat::CRASH => self.crash_count+1,
             _ => self.crash_count,        
        };
        self.queue_len = quelen;
        self.conf_count += 1;
        self.time_elapsed = self.start_time.1.elapsed();
        true
    }
}
