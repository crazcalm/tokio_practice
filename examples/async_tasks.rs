use tokio::task;
use tokio::time;

async fn hello(name: &str) -> String {
    time::sleep(time::Duration::from_secs(5)).await;
    format!("Hello {}", name)
}

#[tokio::main]
async fn main() {
    let greeting_join = task::spawn(hello("Willock"));
    let greeting_join_2 = task::spawn(async {
        time::sleep(time::Duration::from_secs(5)).await;
        "Hello Allen".to_string()
    });

    let greeting = hello("Marcus").await;
    let greeting_join_result = greeting_join.await.unwrap();
    let greeting_join_result_2 = greeting_join_2.await.unwrap();

    println!("{}", greeting);
    println!("{}", greeting_join_result);
    println!("{}", greeting_join_result_2);
}
