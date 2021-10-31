//! ## DataGen
//!
//! provides a random data generator Poll impl

/**
 * MIT License
 *
 * tui-realm - Copyright (C) 2021 Christian Visintin
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
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
        let x = self.data.last().map(|x| x.0 + 1.0).unwrap_or(0.0);
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
