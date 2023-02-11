use tokio::sync::watch;

#[derive(Debug, Clone)]
struct Config {
    pub db_path: String,
}

impl Config {
    fn new(db_path: String) -> Self {
        Config { db_path }
    }

    fn update(&mut self, db_path: String) {
        self.db_path = db_path;
    }
}

async fn listens_for_changes( mut rx: watch::Receiver<Config>) {
    while rx.changed().await.is_ok(){
    let new_config = rx.borrow().clone();

    print!("\n\nNew config is {:#?}", new_config);
    }
}

#[tokio::main]
async fn main() {
    let mut config = Config::new("db_path".to_string());

    let (tx, rx_1) = watch::channel(config.clone());
    let rx_2 = tx.subscribe();

    let handle_1 = tokio::spawn(listens_for_changes(rx_1));
    let handle_2 = tokio::spawn(listens_for_changes(rx_2));

    config.update("new_db_path".to_string());
    tx.send(config).unwrap();

    // closing the channel
    drop(tx);

    handle_1.await.unwrap();
    handle_2.await.unwrap();
}
