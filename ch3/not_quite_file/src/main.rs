use rand::prelude::*; // 공통 트레이트와 타입을 가져온다.

fn one_in(denominator: u32) -> bool {
    thread_rng().gen_ratio(1, denominator) // 쓰레드 로컬 난수 생성기를 사용하여 1/denominator의 확률로 true를 반환
}

#[derive(Debug)]
enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f.state = FileState::Closed;
        f
    }

    
fn read(
    self: &File,
    save_to: &mut Vec<u8>,
) -> Result<usize, String> {
    let mut tmp = self.data.clone();
    let read_length = tmp.len();

    save_to.reserve(read_length); // save_to 벡터에 읽은 데이터를 저장하기 위해 공간을 확보
    save_to.append(&mut tmp); // save_to 벡터에 읽은 데이터를 추가
    Ok(read_length)
}
}

fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;
    Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Closed;
    Ok(f)
}

fn main() {
    let f2_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f2 = File::new_with_data("f2.txt", &f2_data);

    let mut buffer = vec![];

    if f2.read(&mut buffer).is_err() {
        println!("Error checking is working");
    }

    f2 = open(f2).unwrap();
    let f2_length = f2.read(&mut buffer).unwrap();
    f2 = close(f2).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f2);
    println!("{} is {} bytes long", f2.name, f2_length);
    println!("{}", text);

}
