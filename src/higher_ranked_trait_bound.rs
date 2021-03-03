trait Trait<T> {
    fn do_something(&self, value: T);
}

struct TraitImpl;

impl Trait<&i32> for TraitImpl
{
    fn do_something(&self, value: &i32)
    {
        println!("{}", value);
    }
}

// error
/*
fn foo(b: Box<Trait<&i32>>) {
    let x: i32 = 10;
    b.do_something(&x);
}*/

fn bar(b: Box<for<'a> Trait<&'a i32>>) {
    let x: i32 = 10;
    b.do_something(&x);
}

fn main() {
    bar(Box::new(TraitImpl{}));
}
