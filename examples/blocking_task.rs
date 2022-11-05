use std::{thread, time};

use tokio::time::{sleep, Duration};

fn slow_function() -> String {
    thread::sleep(time::Duration::from_secs(10));

    "Finally done".to_string()
}

async fn other_call(num: i32) {
    sleep(Duration::from_secs(1)).await;
    println!("Other Call {}", num);
}

#[tokio::main]
async fn main() {
    let sync_code = tokio::task::spawn_blocking(slow_function);

    let mut tasks = vec![];
    for num in 0..10 {
        let task = tokio::spawn(other_call(num));
        tasks.push(task);
    }

    for task in tasks {
        task.await.unwrap();
    }
    let result = sync_code.await.unwrap();

    println!("{}", result);
}
