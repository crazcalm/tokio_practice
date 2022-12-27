use tokio::sync::oneshot;

async fn double_agent(mut rx_1: oneshot::Receiver<String>, mut rx_2: oneshot::Receiver<String>) {
    let msg = tokio::select! {
        msg_1 = &mut rx_1 => msg_1.unwrap(),
        msg_2 = &mut rx_2 => msg_2.unwrap()
    };
    println!("Spy has received the following msg: {msg}")
}

async fn spy_agency(tx: oneshot::Sender<String>, msg: String) {
    tx.send(msg).unwrap();
}

#[tokio::main]
async fn main() {
    let (tx_1, rx_1) = oneshot::channel();
    let (tx_2, rx_2) = oneshot::channel();

    let agency_1_handle = tokio::spawn(spy_agency(
        tx_1,
        "Organization A: Move to phase two".to_string(),
    ));
    let agency_2_handle = tokio::spawn(spy_agency(
        tx_2,
        "Organization B: The enemy is coming from the hills".to_string(),
    ));
    let spy_handle = tokio::spawn(double_agent(rx_1, rx_2));

    agency_1_handle.await.unwrap();
    agency_2_handle.await.unwrap();
    spy_handle.await.unwrap();
}
