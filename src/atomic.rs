use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use std::thread;

fn main()
{
    let atomic = Arc::new(AtomicI32::new(0));
    let atomic2 = Arc::clone(&atomic);
    
    let handle = thread::spawn(move ||{
        for _ in 0..100000 {
            atomic2.fetch_add(1, Ordering::Relaxed);
        }
    });
    
    for _ in 0..100000 {
        atomic.fetch_add(1, Ordering::Relaxed);
    }
    
    handle.join().unwrap();
    
    println!("{:?}", atomic);
}
