//! ## DataGen
//!
//! provides a random data generator Poll impl

extern crate rand;

use rand::{thread_rng, Rng};

pub struct DataGen<T> {
    max: T,
    min: T,
    data: Vec<T>,
}

impl<T> DataGen<T> {
    pub fn new(min: T, max: T) -> Self {
        Self {
            min,
            max,
            data: Vec::new(),
        }
    }
}

impl DataGen<(f64, f64)> {
    pub fn generate(&mut self) -> Vec<(f64, f64)> {
        let y_max = self.max.1;
        let y_min = self.min.1;
        let x = self.data.last().map_or(0.0, |x| x.0 + 1.0);
        let y = self.get_rand(y_min, y_max);
        self.data.push((x, y));
        self.data.clone()
    }

    fn get_rand(&mut self, min: f64, max: f64) -> f64 {
        let mut rng = thread_rng();
        let min = (min * 10.0) as usize;
        let max = (max * 10.0) as usize;
        rng.gen_range(min..max) as f64 / 10.0
    }
}

impl DataGen<u64> {
    pub fn generate(&mut self) -> Vec<u64> {
        let num = self.get_rand(self.min, self.max);
        self.data.push(num);
        self.data.clone()
    }

    fn get_rand(&mut self, min: u64, max: u64) -> u64 {
        let mut rng = thread_rng();
        rng.gen_range(min..max)
    }
}
