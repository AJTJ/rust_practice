use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
fn main() {
    let a = Arc::new(Mutex::new(String::from("neutral")));
    let b = Arc::new(Mutex::new(String::from("neutral 2")));
    let mut handles = vec![];
    {
        let a = Arc::clone(&a);
        let b = Arc::clone(&b);
        let first_handle = thread::spawn(move || {
            let mut string = a.lock().unwrap();
            thread::sleep(Duration::from_secs(2));
            let mut string2 = b.lock().unwrap();
            *string = String::from("BIG BOY");
            *string2 = String::from("BIG BOY 2")
        });
        handles.push(first_handle);
    }

    println!("FIRST a: {}, b: {}", a.lock().unwrap(), b.lock().unwrap());

    {
        let a = Arc::clone(&a);
        let b = Arc::clone(&b);
        let second_handle = thread::spawn(move || {
            let mut string2 = b.lock().unwrap();
            thread::sleep(Duration::from_secs(2));
            let mut string = a.lock().unwrap();
            *string = String::from("MASSIVE");
            *string2 = String::from("MASSIVE 2")
        });
        handles.push(second_handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("SECOND a: {}, b: {}", a.lock().unwrap(), b.lock().unwrap());
}
