use std::sync::{atomic::AtomicI64, Arc};

fn main() {
    let barrier = Arc::new(std::sync::Barrier::new(2));

    loop {
        let number = Arc::new(AtomicI64::new(0));
        let number2 = Arc::new(AtomicI64::new(0));

        let thread1 = {
            let number = number.clone();
            let number2 = number2.clone();
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                barrier.wait();
                number.store(10, std::sync::atomic::Ordering::Relaxed);
                number2.store(20, std::sync::atomic::Ordering::Relaxed);
            })
        };

        let thread2 = {
            let number = number.clone();
            let number2 = number2.clone();
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                barrier.wait();
                let number = number.load(std::sync::atomic::Ordering::Relaxed);
                let number2 = number2.load(std::sync::atomic::Ordering::Relaxed);

                if number == 0 && number2 == 20 {
                    println!("number = {}", number);
                    println!("number2 = {}", number2);
                }
            })
        };

        thread1.join().unwrap();
        thread2.join().unwrap();
    }
}
