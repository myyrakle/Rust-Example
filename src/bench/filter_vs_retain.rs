#![feature(test)]
#![allow(unused_imports)]
#![allow(dead_code)]

use test::Bencher;
extern crate test;

const N: usize = 1000000;

use rand::prelude::SliceRandom;
use rand::Rng;

#[bench]
fn bench_retain(b: &mut Bencher) {
    let mut vec = vec![];

    for i in 0..N {
        vec.push(i);
    }

    let mut rng = rand::thread_rng();

    vec.shuffle(&mut rng);

    b.iter(|| {
        vec.retain(|&x| x % 2 == 0);
        println!("{:?}", vec);
    });
}

#[bench]
fn bench_filter(b: &mut Bencher) {
    let mut vec = vec![];

    for i in 0..N {
        vec.push(i);
    }

    let mut rng = rand::thread_rng();

    vec.shuffle(&mut rng);

    b.iter(move || {
        let result: Vec<_> = vec.iter().filter(|&x| x % 2 == 0).collect();
        println!("{:?}", result);
    });
}

fn main() {
    let mut vec = vec![];

    for i in 0..N {
        vec.push(i);
    }

    let mut rng = rand::thread_rng();

    vec.shuffle(&mut rng);

    {
        let mut vec = vec.clone();
        let timer = std::time::Instant::now();

        vec.retain(|&x| x % 2 == 0);

        let elapsed = timer.elapsed();

        std::fs::write("dummy.txt", format!("{:?}", vec)).unwrap();
        std::fs::remove_file("dummy.txt").unwrap();

        println!("retain Elapsed: {:?}", elapsed);
    }

    {
        let vec = vec.clone();
        let timer = std::time::Instant::now();

        let vec = vec.into_iter().filter(|&x| x % 2 == 0).collect::<Vec<_>>();

        let elapsed = timer.elapsed();

        std::fs::write("dummy.txt", format!("{:?}", vec)).unwrap();
        std::fs::remove_file("dummy.txt").unwrap();

        println!("filter Elapsed: {:?}", elapsed);
    }
}
