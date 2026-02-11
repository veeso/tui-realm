//! A Taskpool for spawning, tracking and cancelling async ports.

use tokio::runtime::Handle;
use tokio::select;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

#[derive(Debug)]
pub struct TaskPool {
    tracker: TaskTracker,
    handle: Handle,
    // the tracker itself does not cancel tasks on close
    cancel_token: CancellationToken,
}

impl TaskPool {
    pub fn new(handle: Handle) -> Self {
        Self {
            tracker: TaskTracker::new(),
            handle,
            cancel_token: CancellationToken::new(),
        }
    }

    /// Spawn a new future on the [`TaskPool`]s which is tracked and can be cancelled.
    pub fn spawn<F>(&self, fut: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let token = self.cancel_token.clone();
        self.handle.spawn(self.tracker.track_future(async move {
            select! {
                () = fut => {},
                () = token.cancelled() => {}
            }
        }));
    }

    /// Close the tracker and allow [`wait_done`](Self::wait_done) to exit.
    ///
    /// Does not prevent adding new tasks.
    /// Does not cancel any tasks.
    pub fn close(&self) {
        self.tracker.close();
    }

    /// Cancel all tracked tasks.
    pub fn cancel_all(&self) {
        self.cancel_token.cancel();
    }

    /// Wait until all tasks have finished.
    ///
    /// NOTE: this will wait infinitely until the task tracker is closed!
    #[allow(dead_code)]
    pub async fn wait_done(&self) {
        self.tracker.wait().await;
    }

    /// Close the Tracker, cancel all tasks in the tracker and wait for all tasks to settle.
    #[allow(dead_code)]
    pub async fn cancel_and_wait(&self) {
        self.close();
        self.cancel_all();
        self.wait_done().await;
    }
}

#[cfg(test)]
mod tests {
    use alloc::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;

    use tokio::runtime::Handle;
    use tokio::time::sleep;

    use crate::listener::task_pool::TaskPool;

    #[tokio::test]
    async fn should_spawn_and_close() {
        let taskpool = TaskPool::new(Handle::current());
        assert!(taskpool.tracker.is_empty());
        assert!(!taskpool.tracker.is_closed());

        let active = Arc::new(AtomicUsize::new(0));

        let active_t = active.clone();
        taskpool.spawn(async move {
            sleep(Duration::from_millis(2)).await;
            active_t.fetch_add(1, Ordering::Relaxed);
        });

        taskpool.close();
        taskpool.wait_done().await;

        assert_eq!(active.load(Ordering::Relaxed), 1);
        assert!(taskpool.tracker.is_empty());
        assert!(taskpool.tracker.is_closed());
    }

    #[tokio::test]
    async fn should_cancel() {
        let taskpool = TaskPool::new(Handle::current());
        assert!(taskpool.tracker.is_empty());
        assert!(!taskpool.tracker.is_closed());
        // note: it seemingly does not count the main async function of the runtime
        assert_eq!(Handle::current().metrics().num_alive_tasks(), 0);

        let active = Arc::new(AtomicUsize::new(0));

        let active_t = active.clone();
        taskpool.spawn(async move {
            active_t.fetch_add(1, Ordering::Relaxed);
            sleep(Duration::MAX).await;
        });

        // just to be sure that the other tasks gets executed too
        sleep(Duration::from_millis(10)).await;

        assert_eq!(Handle::current().metrics().num_alive_tasks(), 1);

        taskpool.cancel_and_wait().await;

        assert_eq!(active.load(Ordering::Relaxed), 1);
        assert!(taskpool.tracker.is_empty());
        assert!(taskpool.tracker.is_closed());
        assert_eq!(Handle::current().metrics().num_alive_tasks(), 0);
    }
}
