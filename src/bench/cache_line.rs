/*
```
[dependencies]
rand = "0.8.5"
```
*/

pub struct Timer {
    start: std::time::Instant,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            start: std::time::Instant::now(),
        }
    }

    pub fn elapsed(&self) -> std::time::Duration {
        self.start.elapsed()
    }

    pub fn elapsed_as_millis(&self) -> u128 {
        self.elapsed().as_millis()
    }
}

const ARRAY_SIZE: usize = 100_000_000;

fn do_something_with_indices(array: &Vec<i32>, indices: &Vec<usize>) -> i64 {
    let mut sum = 0;

    for i in indices {
        let value = array[*i];
        sum += value as i64;
    }

    sum
}

fn get_randome_indices() -> Vec<usize> {
    use rand::seq::SliceRandom;
    let mut indices = (0..ARRAY_SIZE).collect::<Vec<usize>>();
    let mut rng = rand::thread_rng();
    indices.shuffle(&mut rng);
    indices
}

fn get_sequencial_indices() -> Vec<usize> {
    (0..ARRAY_SIZE).collect::<Vec<usize>>()
}

fn main() {
    let mut array = vec![0; ARRAY_SIZE];
    for i in 0..ARRAY_SIZE {
        array[i] = i as i32;
    }

    println!("Array Size: {}", ARRAY_SIZE);

    let indices = get_sequencial_indices();
    let timer = Timer::new();
    do_something_with_indices(&array, &indices);
    println!("Seq Access => Elapsed: {}ms", timer.elapsed_as_millis());

    let indices = get_randome_indices();
    let timer = Timer::new();
    do_something_with_indices(&array, &indices);
    println!("Randome Access => Elapsed: {}ms", timer.elapsed_as_millis());
}
