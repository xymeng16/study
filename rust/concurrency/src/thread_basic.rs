use std::thread;
use std::time::Duration;

pub fn thread_basic_join() {
    let handle = thread::spawn(|| {
       for i in 1..10 {
           println!("hi number {} from the spawned thread", i);
           thread::sleep(Duration::from_millis(10));
       }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap_or_else(|err|println!("{:?}", err)); // why unwrap here?
}

pub fn thread_basic_move() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // drop(v);

    handle.join().unwrap();
}