#![no_main]

#[unsafe(no_mangle)]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    const HELLO: &'static str = "Hello, World!\n\0";
    unsafe {
        libc::printf(HELLO.as_ptr() as *const _);
    }

    0 // 성공적인 종료
}

// to run
// cargo run --release 
