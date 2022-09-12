#![allow(unused_variables)]

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    
fn read(
    self: &File,
    save_to: &mut Vec<u8>,
) -> usize {
    let mut tmp = self.data.clone();
    let read_length = tmp.len();

    save_to.reserve(read_length); // save_to 벡터에 읽은 데이터를 저장하기 위해 공간을 확보
    save_to.append(&mut tmp); // save_to 벡터에 읽은 데이터를 추가
    read_length
}
}

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

fn main() {
    // let mut f2 = File {
    //     name: String::from("f2.txt"),
    //     data: vec![114, 117, 115, 116, 33],
    // };
    let f1 = File::new("f1.txt");
    let mut buffer_f1 = vec![];
    let f1_length = f1.read(&mut buffer_f1);
    println!("{:?}", f1);
    println!("{} is {} bytes long", f1.name, f1_length);
    
    let f2_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f2 = File::new_with_data("f2.txt", &f2_data);

    let mut buffer = vec![];
    let f2_length = f2.read(&mut buffer);
    close(&mut f2);

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f2);
    println!("{} is {} bytes long", f2.name, f2_length);
    println!("{}", text);

}
