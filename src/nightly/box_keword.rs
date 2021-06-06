#![feature(box_syntax)]

fn main() {
    // let a = Box::new(4)와 동일
    let a = box 4;
    // let b = Box::new(5) 와 동일
    let b = box 5;
    
    println!("{}", *a+*b);
}
