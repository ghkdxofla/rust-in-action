//! 한 번에 한 단계 씩 파일을 시뮬레이트 한다.

// use rand::prelude::*; // 공통 트레이트와 타입을 가져온다.
#![allow(dead_code)]

use std::fmt;
use std::fmt::{Display};

// fn one_in(denominator: u32) -> bool {
//     thread_rng().gen_ratio(1, denominator) // 쓰레드 로컬 난수 생성기를 사용하여 1/denominator의 확률로 true를 반환
// }

#[derive(Debug, PartialEq)]
pub enum FileState {
    Open,
    Closed,
}

// impl Trait for Type 형태로 구현한다.
impl Display for FileState {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

/// 아마도 파일 시스템에 있을
/// '파일'을 나타낸다.
#[derive(Debug)]
pub struct File {
    pub name: String,
    data: Vec<u8>, // data는 여전히 이 크레이트를 임포트 해도 비공개로 남는다.
    pub state: FileState,
}

impl File {
    /// 빈 `File`을 새로 만든다.
    /// 
    /// # Examples
    /// 
    /// ```
    /// let f = File::new("foo.txt");
    /// ```
    pub fn new(name: &str) -> File { // 메서드는 구조체가 pub라 하더라도 공개 여부를 명시적으로 지정해야 한다.
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

    /// 파일 길이를 바이트로 반환한다.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// 파일 이름을 반환한다.
    pub fn name(&self) -> String {
        self.name.clone()
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

impl Display for File {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
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
    let f2_length = f2.len();
    f2 = close(f2).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f2);
    println!("{}", f2);
    println!("{} is {} bytes long", f2.name(), f2_length);
    println!("{}", text);
}
