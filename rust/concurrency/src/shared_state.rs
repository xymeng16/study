use std::sync::{Mutex, Arc};
use std::thread;

pub fn mutex_single() {
    let m = Mutex::new(5); // a smart pointer

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}


pub fn mutex_multi() {
    let counter = Arc::new(Mutex::new(0)); // Rc is not thread-safe, so use Arc instead
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
           let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}