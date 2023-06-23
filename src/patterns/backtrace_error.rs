use std::{backtrace::Backtrace, env};

#[derive(Debug)]
struct FooError {
    _message: String,
}

fn generate_foo_error() -> Result<(), FooError> {
    Err(FooError {
        _message: "FooError".to_string(),
    })
}

fn do_something() -> Result<(), FooError> {
    generate_foo_error()?;
    Ok(())
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    do_something().unwrap();
}
