use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

pub fn channels_implementation(data: Vec<String>) {
    let vec_length = data.len();

    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let mut children = Vec::new();

    for task in data {
        let thread_tx = tx.clone();

        let child = thread::spawn(move || {
            println!("{}", task);

            thread_tx.send(02).unwrap();
        });

        //println!("The task {:?} was successfully executed", task);
        children.push(child);
    }

    let mut ids = Vec::with_capacity(vec_length);
    for _ in 0..vec_length {
        ids.push(rx.recv());
    }

    for child in children {
        child.join().expect("oops! the child thread panicked");
    }

    println!("{:?}", ids);
}
