use rand;
use rand::distributions::{Distribution, Uniform};

pub fn random(n: usize) -> usize {
    let range = Uniform::from(0..n);
    let mut rng = rand::thread_rng();
    range.sample(&mut rng)
}
