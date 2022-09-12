struct Hostname(String); // newtype 정의

fn connect(host: Hostname) {
    println!("Connecting to {}", host.0);
}

fn main() {
    let ordinary_string = String::from("www.rust-lang.org");
    let host = Hostname(ordinary_string.clone());
    connect(ordinary_string);
}
