use std::marker::PhantomData;

struct Foo<T> 
{
    this: Option<Box<Foo<T>>>,
    _phantom: PhantomData<T>, //사용한척
}

fn main()
{
    let foo = Foo::<i32>{
       this: None, 
       _phantom: PhantomData
    };
}
