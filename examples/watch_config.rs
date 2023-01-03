use std::sync::Arc;

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

async fn listens_for_changes(rx: Arc<watch::Receiver<Config>>) {
    loop {
        if rx.has_changed().unwrap() {
            break;
        }
    }
    let new_config = rx.borrow().clone();

    print!("\n\nNew config is {:#?}", new_config);
}

#[tokio::main]
async fn main() {
    let mut config = Config::new("db_path".to_string());

    let (tx, rx) = watch::channel(config.clone());
    let rx_arc = Arc::new(rx);

    let handle_1 = tokio::spawn(listens_for_changes(rx_arc.clone()));
    let handle_2 = tokio::spawn(listens_for_changes(rx_arc));

    config.update("new_db_path".to_string());
    tx.send(config).unwrap();

    handle_1.await.unwrap();
    handle_2.await.unwrap();
}
