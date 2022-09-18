#![allow(unused_variables)]

#[derive(Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(sat_id: CubeSat) -> CubeSat {
    println!("{:?}: {:?}", sat_id, StatusMessage::Ok);
    sat_id
}

fn main() {
    let sat_a = CubeSat { id: 0 };
    /**
     * 이전 커밋에서는 가능했던 이유: 
     * Rust는 Primitive type은 복사를 구현함(Copy semantics)
     * 아래의 타입은 이동을 한다(Move semantics)
     */

    let sat_b = CubeSat { id: 1 };
    let sat_c = CubeSat { id: 2 };

    let sat_a = check_status(sat_a);
    let sat_b = check_status(sat_b);
    let sat_c = check_status(sat_c);

    // '대기 중' ...
    let sat_a = check_status(sat_a);
    let sat_b = check_status(sat_b);
    let sat_c = check_status(sat_c);
}
