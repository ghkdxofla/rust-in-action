fn greet_world() {
    println!("Hello, world!"); // !는 매크로
    let southern_germany = "grüß gott";
    let korean = "안녕, 세상!";
    let regions = [southern_germany, korean];

    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn main() {
    greet_world();
}
