use std::marker::PhantomData;

struct Foo<T: ?Sized>
{
    _ph: PhantomData<T>,
}

struct A;
struct B;

trait Boom {}
impl Boom for A {}
impl Boom for B {}

fn main()
{
    let _foo = Foo::<i32>{
        _ph: PhantomData
    };
    
    let _foo = Foo::<[i32]>{
        _ph: PhantomData
    };
    
    let _foo = Foo::<dyn Boom>{
        _ph: PhantomData
    };
}
