//nightly functions

#![feature(unboxed_closures, fn_traits)]

#[derive(Clone, Copy)]
struct Boom{}

impl Fn<()> for Boom {
    extern "rust-call" fn call(&self, _args: ()) ->()
    {
        println!("Boom!");
    }
}

impl FnMut<()> for Boom {
    extern "rust-call" fn call_mut(&mut self, _args: ()) ->()
    {
        println!("Boom! mut");
    }
}

impl FnOnce<()> for Boom {
    type Output = ();
    extern "rust-call" fn call_once(self, _args: ()) ->()
    {
        println!("Boom! once");
    }
}

fn main()
{
    let boom = Boom{};
    boom();
}
