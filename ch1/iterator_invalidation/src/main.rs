fn main() {
    let mut letters = vec![
        "a", "b", "b"
    ];

    for letter in letters {
        println!("{}", letter);
        letters.push(letter.clone()); // 글자 복제 후 벡터 끝에 붙이는 작업
    }
}

// cargo run 결과
/*
error[E0382]: borrow of moved value: `letters`
 --> src\main.rs:8:9
  |
2 |     let mut letters = vec![
  |         ----------- move occurs because `letters` 
has type `Vec<&str>`, which does not implement the `Copy` trait
...
6 |     for letter in letters {
  |                   ------- `letters` moved due to this implicit call to `.into_iter()`
7 |         println!("{}", letter);
8 |         letters.push(letter.clone()); // 글자 복제
 후 벡터 끝에 붙이는 작업
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ value borrowed here after move
  |
note: this function takes ownership of the receiver `self`, which moves `letters`
help: consider iterating over a slice of the `Vec<&str>`'s content to avoid moving into the `for` loop      
  |
6 |     for letter in &letters {
  |                   +

For more information about this error, try `rustc --explain E0382`.
error: could not compile `iterator_invalidation` due to previous error
*/
