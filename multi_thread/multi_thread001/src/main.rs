use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;

//mpsc::channel() メソッドを使って、新たなチャネルを生成し、()が10個戻ってくるのを待機するだけのコード
fn main() {
    let data = Arc::new(Mutex::new(0));

    let (tx, rx) = mpsc::channel();

    for _ in 0..10 {
        let (data, tx) = (data.clone(), tx.clone());

        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            *data += 1;

            tx.send(()).unwrap();
        });
    }

    for _ in 0..10 {
        rx.recv().unwrap();
    }
}