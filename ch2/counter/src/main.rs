use std::time::{Duration, Instant};

fn main() {
    let mut count = 0;
    let time_limit = Duration::new(1, 0);
    let start = Instant::now();

    while (Instant::now() - start) < time_limit {
        count += 1;
    }
    println!("{}", count);

    loop {
        println!("Infinity loop");
        break;
    }

    'outer: for x in 0.. { // for loop에 label을 지정할 수 있다
        for y in 0.. {
            for z in 0.. {
                if x + y + z > 1000 {
                    println!("{}", x + y + z);
                    break 'outer; // 지정한 label에 break
                }
            }
        }
    }
}
