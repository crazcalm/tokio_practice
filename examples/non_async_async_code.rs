use tokio::time;

async fn hello(name: &str) -> String {
    time::sleep(time::Duration::from_secs(5)).await;
    format!("Hello {}", name)
}

#[tokio::main]
async fn main() {
    let greeting = hello("Marcus").await;
    let greeting_2 = hello("Willock").await;
    let greeting_3 = hello("Allen").await;

    println!("{}", greeting);
    println!("{}", greeting_2);
    println!("{}", greeting_3);
}
