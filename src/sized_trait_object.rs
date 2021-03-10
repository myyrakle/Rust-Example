struct A;
struct B;

trait Boom: Sized {}
impl Boom for A {}
impl Boom for B {}

fn main()
{
    let _foo: &dyn Boom = &A; //error
}
