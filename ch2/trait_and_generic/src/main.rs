/**
 * Trait: 인터페이스, 프로토콜 또는 계약과 비슷한 언어 기능
 * 객체 지향 프로그래밍의 추상 기본 클래스
 * 함수형 프로그래밍의 타입 클래스
 * 러스트의 모든 연산자는 트레이트의 메서드에 대한 간편 문법
 * 즉, 연산자 오버로딩 지원
 */

use std::ops::{Add};
use std::time::{Duration};

fn add<T: Add<Output = T>>(i: T, j: T) -> T {
    i + j
}

fn main() {
    let floats = add(1.2, 3.4);
    let ints = add(10, 20);
    let durations = add(
        Duration::new(5, 0),
        Duration::new(10, 0)
    );

    println!("{} {} {:?}", floats, ints, durations);
}
