// use std::convert::TryInto; // 2021 에디션 이후에는 prelude에 추가되어 있다

fn main() {
    let a: i32 = 10;
    let b: u16 = 100;

    let b_ = b.try_into().unwrap();

    if a < b_ {
        println!("Ten is less than one hundred.");
    }
}
 