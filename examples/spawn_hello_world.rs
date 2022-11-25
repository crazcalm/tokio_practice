use tokio::task::JoinHandle;

async fn hello(name: &str) -> String {
    format!("Hello {}", name)
}

#[tokio::main]
async fn main() {
    let join_handle: JoinHandle<String> = tokio::spawn(hello("Marcus"));
    let value = join_handle.await.unwrap();
    println!("{}", value);
}
