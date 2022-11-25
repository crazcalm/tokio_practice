use std::sync::Arc;
use tokio::sync::{Barrier, BarrierWaitResult, Notify};
use tokio::time::{sleep, Duration};

async fn example(barrier: Arc<Barrier>, note: Arc<Notify>) -> BarrierWaitResult {
    println!("before wait");

    // This is he Rendezvous point. We will wiat until all of the
    // threads (with reference to the barrier size) has reached this point
    // before we procceed.
    let wait_result = barrier.wait().await;
    println!("after wait");

    let is_leader = wait_result.is_leader();

    if is_leader {
        note.notify_one();
    }

    wait_result
}

#[tokio::main]
async fn main() {
    let mut handles = Vec::with_capacity(100);
    let barrier = Arc::new(Barrier::new(10));
    let note = Arc::new(Notify::new());

    note.notify_one();
    // Warning: The barrier keeps an active count of how many threads are at the
    // Rendezvous point.
    //
    // If this for loop has a number less than the barrier size,
    // then your code will wait forever.
    //
    // If this for loop has a number greater than the barrier size, then
    // the first batch of threads will work as expected, but the following
    // batches will not because now you have to deal with the rate of change
    // between exiting threads leaving the Rendezvous point while new threads
    // are entering the Rendezvous point (set the for loop to 100 to see example)
    for count in 0..100 {
        let barrier_clone = barrier.clone();
        let note_clone = note.clone();

        if count % 10 == 0 {
            // Making sure only ten tasks get sent to the barrier
            note.notified().await;

            // Giving the Barrier some time to close before the next tasks is sent
            sleep(Duration::from_millis(1)).await;
        }

        // The same messages will be printed together
        // You will NOT see any interleaving.
        handles.push(tokio::spawn(example(barrier_clone, note_clone)))
    }

    // Will not resolve until all "after wait" messages have been printed
    let mut num_leaders = 0;
    for handle in handles {
        let wait_result = handle.await.unwrap();
        if wait_result.is_leader() {
            num_leaders += 1;
        }
    }

    dbg!(num_leaders);
}
