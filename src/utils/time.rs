//! Time abstraction layer for both std and no_std environments
//!
//! This module provides a `Clock` trait that abstracts time operations,
//! allowing the application to work in both std and no_std environments.

// Duration is available in core::time since Rust 1.66
#[cfg(feature = "std")]
pub use std::time::Duration;

#[cfg(not(feature = "std"))]
pub use core::time::Duration;

#[cfg(feature = "std")]
pub use std::time::Instant;

/// A trait for clock implementations that provide time measurement capabilities.
///
/// This trait allows the application to be generic over the time source,
/// enabling support for both std (using `std::time::Instant`) and no_std
/// environments (using custom hardware timers or monotonic counters).
pub trait Clock {
    /// The instant type used by this clock.
    ///
    /// Must be Copy and Clone for easy passing around.
    /// Must support comparison to measure elapsed time.
    type Instant: Copy + Clone + PartialEq + PartialOrd;

    /// Get the current instant from this clock.
    fn now(&self) -> Self::Instant;

    /// Calculate the duration elapsed since the given instant.
    ///
    /// Returns `Duration::ZERO` if the instant is in the future.
    fn elapsed(&self, start: Self::Instant) -> Duration;

    /// Check if a duration has elapsed since the given instant.
    #[inline]
    fn has_elapsed(&self, start: Self::Instant, duration: Duration) -> bool {
        self.elapsed(start) >= duration
    }
}

/// Standard library clock implementation using `std::time::Instant`.
///
/// This is the default clock implementation for std environments.
#[cfg(feature = "std")]
#[derive(Debug, Clone, Copy, Default)]
pub struct StdClock;

#[cfg(feature = "std")]
impl Clock for StdClock {
    type Instant = Instant;

    #[inline]
    fn now(&self) -> Self::Instant {
        Instant::now()
    }

    #[inline]
    fn elapsed(&self, start: Self::Instant) -> Duration {
        start.elapsed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "std")]
    #[test]
    fn test_std_clock() {
        let clock = StdClock;
        let start = clock.now();
        
        // Small delay to ensure time passes
        std::thread::sleep(Duration::from_millis(10));
        
        let elapsed = clock.elapsed(start);
        assert!(elapsed >= Duration::from_millis(10));
        assert!(clock.has_elapsed(start, Duration::from_millis(5)));
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_instant_ordering() {
        let clock = StdClock;
        let instant1 = clock.now();
        std::thread::sleep(Duration::from_millis(1));
        let instant2 = clock.now();
        
        assert!(instant2 > instant1);
    }
}
