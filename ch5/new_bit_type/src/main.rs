fn main() {
    let a: f32 = 42.42;
    let frankentype: u32 = unsafe { std::mem::transmute(a) }; // type 변환

    println!("{}", frankentype);
    println!("{:032b}", frankentype);

    let b: f32 = unsafe { std::mem::transmute(frankentype) }; // unsafe는 프로그램의 메모리 안정성을 보장할 수 없음을 
    println!("{}", b);
    assert_eq!(a, b);
}
