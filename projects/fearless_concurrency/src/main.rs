use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
fn main() {
    // 说的是新建线程,线程阻塞
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // for i in 1..5 {
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // handle.join().unwrap();

    // 这部分是move强制转换所有权
    // let v = vec![1, 2, 3];
    // let handle = thread::spawn(move || println!("Here's a vector: {:?}", v));

    // handle.join().unwrap();

    // let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx.send(val).unwrap();
    // });

    // let recevied = rx.recv().unwrap();
    // println!("Got {}", recevied);

    // 这里讲的多个producer的通信
    // let (tx, rx) = mpsc::channel();

    // let tx1 = mpsc::Sender::clone(&tx);
    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("thread"),
    //     ];

    //     for val in vals {
    //         tx1.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("more"),
    //         String::from("messages"),
    //         String::from("for"),
    //         String::from("you"),
    //     ];

    //     for val in vals {
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // for received in rx {
    //     print!("Got: {}\n", received);
    // }

    // 互斥锁Mutex
    // let m = Mutex::new(5);
    // {
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    // }

    // println!("m = {:?}", m);

    let m = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {}
}
