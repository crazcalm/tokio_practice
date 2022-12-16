use std::sync::Arc;

use tokio::sync::RwLock;

async fn read_from_document(id: i32, document: Arc<RwLock<String>>) {
    let reader = document.read().await;

    println!("reader_{}: {}", id, *reader);
}

async fn write_to_document(locked_document: Arc<RwLock<String>>, new_string: &str) {
    let mut document = locked_document.write().await;
    document.push_str(new_string);
    document.push_str(" ");
}

#[tokio::main]
async fn main() {
    let mut handles = Vec::new();
    let locked_document = Arc::new(RwLock::new("".to_string()));

    for word in "I can read and write a b c d e f g h i j k l m n o p q r s t u v w x y z"
        .split_whitespace()
    {
        handles.push(tokio::spawn(read_from_document(1, locked_document.clone())));
        handles.push(tokio::spawn(write_to_document(
            locked_document.clone(),
            word,
        )));
        handles.push(tokio::spawn(read_from_document(2, locked_document.clone())));
        handles.push(tokio::spawn(read_from_document(3, locked_document.clone())));
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
