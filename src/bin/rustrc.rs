use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx1, rx) = mpsc::channel();
    let tx2 = mpsc::Sender::clone(&tx1);

    thread::spawn(move || {
        println!("Sending message from thread 1");
        tx1.send(String::from("Greeting from thread 1")).unwrap();
    });

    thread::spawn(move || {
        println!("Sending message from thread 2");
        tx2.send(String::from("Greeting from thread 2")).unwrap();
    });

    for recvd in rx {
        println!("Received: {}", recvd);
    }
}