use std::sync::{atomic::AtomicI64, Arc};

fn main() {
    let x = Arc::new(AtomicI64::new(0));
    let y = Arc::new(AtomicI64::new(0));

    let thread1 = {
        let x = x.clone();
        let y = y.clone();
        std::thread::spawn(move || {
            let y_value = y.load(std::sync::atomic::Ordering::Relaxed);
            x.store(y_value, std::sync::atomic::Ordering::Relaxed);

            println!("y_value = {}", y_value);
        })
    };

    let thread2 = {
        let x = x.clone();
        let y = y.clone();
        std::thread::spawn(move || {
            let x_value = x.load(std::sync::atomic::Ordering::Relaxed);
            y.store(4444, std::sync::atomic::Ordering::Relaxed);

            println!("x_value = {}", x_value);
        })
    };

    thread1.join().unwrap();
    thread2.join().unwrap();
}
