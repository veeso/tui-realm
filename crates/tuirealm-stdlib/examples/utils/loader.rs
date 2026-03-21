//! ## Loader
//!
//! provides a loader generator

pub struct Loader {
    progress: f64,
}

impl Default for Loader {
    fn default() -> Self {
        Self { progress: 0.0 }
    }
}

impl Loader {
    pub fn load(&mut self) -> f64 {
        let new = self.progress + 0.01;
        if new >= 1.0 {
            self.progress = 0.0;
        } else {
            self.progress = new;
        }
        self.progress
    }
}
