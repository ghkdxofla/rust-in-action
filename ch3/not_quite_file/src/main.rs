#![allow(unused_variables)]

type File = String;

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

#[allow(dead_code)]
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! { // !는 Never 타입. 함수 호출 위치로 돌아오지 않으며, 중단될 것을 보장한다.
    unimplemented!() // 중단 시키는 매크로
}

fn main() {
    let mut f1 = File::from("f1.txt");
    open(&mut f1);
    //read(f1, vec![]);
    close(&mut f1);
}
