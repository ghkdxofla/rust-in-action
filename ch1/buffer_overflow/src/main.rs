fn main() {
    let fruit = vec!['@', '&', '*'];

    let buffer_overflow = fruit[4];
    assert_eq!(buffer_overflow, '#');
}

// cargo run 결과
/*
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 4', src\main.rs:4:27
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error: process didn't exit successfully: `target\debug\buffer_overflow.exe` (exit code: 101)
*/
