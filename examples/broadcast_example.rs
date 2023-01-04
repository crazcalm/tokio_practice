use std::sync::Arc;

use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let (tx, _rx) = broadcast::channel(100);

    let tx_clone_1 = Arc::new(tx);
    let tx_clone_2 = tx_clone_1.clone();
    let tx_clone_3 = tx_clone_1.clone();

    let mut rx_clone_1 = tx_clone_1.subscribe();
    let mut rx_clone_2 = tx_clone_1.subscribe();
    let mut rx_clone_3 = tx_clone_2.subscribe();

    tx_clone_1.send("msg 1".to_string()).unwrap();
    tx_clone_2.send("msg 2".to_string()).unwrap();
    tx_clone_3.send("msg_2".to_string()).unwrap();

    let msg_1 = rx_clone_1.recv().await.unwrap();
    assert_eq!(msg_1, rx_clone_2.recv().await.unwrap());
    assert_eq!(msg_1, rx_clone_3.recv().await.unwrap());

    let msg_2 = rx_clone_1.recv().await.unwrap();
    assert_eq!(msg_2, rx_clone_2.recv().await.unwrap());
    assert_eq!(msg_2, rx_clone_3.recv().await.unwrap());

    let msg_3 = rx_clone_1.recv().await.unwrap();
    assert_eq!(msg_3, rx_clone_2.recv().await.unwrap());
    assert_eq!(msg_3, rx_clone_3.recv().await.unwrap());
}
