#[allow(arithmetic_overflow)]

fn main() {
    let (a, b) = (200, 200);
    let c: u8 = a + b;
    println!("200 + 200 = {}", c); // rustc -O main.rs && main 으로 플래그 설정 시 200 + 200 = 144라는 결과를 출력한다
}
