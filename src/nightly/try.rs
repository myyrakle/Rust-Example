#![feature(try_blocks)]

fn foo() -> Option<()>
{
    None
}

fn bar() -> Option<()>
{
    Some(())
}

fn main() {
    let result: Option<()> =
    try {
        foo()?;
        bar()?;
    };
    
    println!("{:?}", result);
}
