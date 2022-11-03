use tokio::sync::{Semaphore, TryAcquireError};

#[tokio::main]
async fn main() {
    let semaphore = Semaphore::new(3);

    let _a_permit = semaphore.acquire().await.unwrap();
    let _two_permits = semaphore.acquire_many(2).await.unwrap();

    assert_eq!(semaphore.available_permits(), 0);

    let permit_attempt = semaphore.try_acquire();
    assert_eq!(permit_attempt.err().unwrap(), TryAcquireError::NoPermits);
}
