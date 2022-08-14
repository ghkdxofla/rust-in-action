#[derive(Debug)] // enum 출력 시, println! 매크로 사용할 수 있도록 설정
enum Cereal {
    Barley,
    Millet,
    Rice,
    Rye,
    Spelt,
    Wheat,
}

fn main() {
    let mut grains: Vec<Cereal> = vec![]; // 빈 벡터 정의. Mutable
    grains.push(Cereal::Rye);
    drop(grains);
    println!("{:?}", grains);
}

// cargo run 결과
/*
error[E0382]: borrow of moved value: `grains`
  --> src\main.rs:15:22
   |
12 |     let mut grains: Vec<Cereal> = vec![]; // 빈 벡터 정의. Mutable
   |         ---------- move occurs because `grains` has type `Vec<Cereal>`, which does not implement the `Copy` trait
13 |     grains.push(Cereal::Rye);
14 |     drop(grains);
   |          ------ value moved here
15 |     println!("{:?}", grains);
   |                      ^^^^^^ value borrowed here after move
   |
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
*/
