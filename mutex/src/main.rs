use std::sync::Mutex;
fn main() {
    let m = Mutex::new(5);

    {
        // RETURNS: a MutexGuard wrapped in a LockResult
        let mut num = m.lock().unwrap();
        *num = 6

        //Drop impl (on MutexGuard) is used automatically once the MutexGuard goes out of scope
    }

    println!("m = {:?}", m);
}
