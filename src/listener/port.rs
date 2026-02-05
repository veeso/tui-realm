//! This module exposes the poll wrapper for usage in Ports

#[cfg(feature = "async-ports")]
mod async_p;
mod sync;

#[cfg(feature = "async-ports")]
#[cfg_attr(docsrs, doc(cfg(feature = "async-ports")))]
pub use self::async_p::AsyncPort;
pub use self::sync::SyncPort;

#[cfg(test)]
mod test {
    use std::time::{Duration, Instant};

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::mock::{MockEvent, MockPoll};

    #[test]
    fn test_single_listener() {
        let mut listener =
            SyncPort::<MockEvent>::new(Box::new(MockPoll::default()), Duration::from_secs(5), 1);
        assert!(listener.next_poll() <= Instant::now());
        assert_eq!(listener.should_poll(), true);
        assert!(listener.poll().ok().unwrap().is_some());
        listener.calc_next_poll();
        assert_eq!(listener.should_poll(), false);
        assert_eq!(*listener.interval(), Duration::from_secs(5));
    }

    #[tokio::test]
    #[cfg(feature = "async-ports")]
    async fn test_single_async_listener() {
        use crate::mock::MockPollAsync;

        let mut listener = AsyncPort::<MockEvent>::new(
            Box::new(MockPollAsync::default()),
            Duration::from_secs(5),
            1,
        );
        assert!(listener.next_poll() <= Instant::now());
        assert_eq!(listener.should_poll(), true);
        assert!(listener.poll().await.ok().unwrap().is_some());
        listener.calc_next_poll();
        assert_eq!(listener.should_poll(), false);
        assert_eq!(*listener.interval(), Duration::from_secs(5));
    }
}
