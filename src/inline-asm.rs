#![feature(asm)]

fn main() {
    let l = 3;
    let r = 7;
    let result: i32;

    unsafe {
        asm!(
            "mov rax, {1}", // l 할당
            "mul {2}", // l x r 곱셈
            "mov {0}, rax", // 결과 저장
            out(reg) result,
            in(reg) l,
            in(reg) r,
        );
    }

    println!("{}", result);
}
