use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use std::thread;

fn main()
{
    let atomic = Arc::new(AtomicI32::new(1));
    let atomic2 = Arc::clone(&atomic);
    
    let handle = thread::spawn(move ||{
        for _ in 1..100000 {
            let current = atomic2.load(Ordering::Acquire);
            atomic2.store(current+1, Ordering::Release);
        }
    });
    
    for _ in 1..100000 {
        let current = atomic.load(Ordering::Acquire);
        atomic.store(current+1, Ordering::Release);
    }
    
    handle.join().unwrap();
    
    println!("{:?}", atomic);
}
