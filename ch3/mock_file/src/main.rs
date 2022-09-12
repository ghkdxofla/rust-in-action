#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

fn main() {
    let f1 = File {
        name: String::from("f1.txt"), // 슬라이스 문자열로부터 소유한 문자열 생성
        data: Vec::new(), // vec! 매크로가 빈 파일을 시뮬레이트
    };

    let f1_name = &f1.name;
    let f1_length = &f1.data.len();

    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);
}
