use tokio::sync::mpsc;

async fn student(id: i32, tx: mpsc::Sender<String>) {
    println!("{}", format!("Student {id} is getting their hw"));
    tx.send(format!("Student {id}'s homework")).await.unwrap();
}

async fn teacher(mut rx: mpsc::Receiver<String>) -> Vec<String> {
    let mut homework = Vec::new();

    while let Some(student_hw) = rx.recv().await {
        println!("Received homework: {}", &student_hw);
        homework.push(student_hw);
    }

    homework
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(100);

    //let tx_arc = Arc::new(tx);

    let teacher_handle = tokio::spawn(teacher(rx));

    let mut student_handles = Vec::new();
    for student_id in 0..=10 {
        student_handles.push(tokio::spawn(student(student_id, tx.clone())));
    }

    for handle in student_handles {
        handle.await.unwrap();
    }

    // droping the last receiver from memory
    drop(tx);

    let homework = teacher_handle.await.unwrap();

    println!("\n\nList of received homework: {:#?}", homework);
}
