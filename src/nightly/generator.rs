#![feature(generators, generator_trait)]

use std::ops::Generator;
use std::pin::Pin;

fn main() {
    let mut generator = || {
        let mut count = 0;
        loop {
            count += 1;
            yield count; //반환/
        }
    };
    
    let a = Pin::new(&mut generator).resume(());
    let b = Pin::new(&mut generator).resume(());
    let c = Pin::new(&mut generator).resume(());
    
    println!("{:?} {:?} {:?}", a, b, c);
}
