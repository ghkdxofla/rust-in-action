use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main() {
    let a = 10; // 스택에 정수 생성
    let b = Box::new(20); // 힙에 정수 생성(boxed integer)
    let c = Rc::new(Box::new(30)); // 참조 카운터에 박스된 정수를 담는다
    let d = Arc::new(Mutex::new(40)); // atomic 참조 카운터에 담김. mutual exclusion 잠금 방식으로 보호 받는다
    println!("a: {:?}, b: {:?}, c: {:?}, d: {:?}", a, b, c, d);
}
