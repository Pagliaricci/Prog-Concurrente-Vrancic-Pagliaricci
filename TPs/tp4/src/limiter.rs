use tokio::sync::Semaphore;
use std::sync::Arc;

#[derive(Clone)]
pub struct FileLimiter {
    semaphore: Arc<Semaphore>,
}

impl FileLimiter {
    pub fn new(max_permits: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(max_permits)),
        }
    }

    pub fn try_acquire(&self) -> Option<tokio::sync::OwnedSemaphorePermit> {
        self.semaphore.clone().try_acquire_owned().ok()
    }
}
