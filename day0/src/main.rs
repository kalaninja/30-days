use std::io;

fn main() {
    println!("Hello, World.");

    let mut str = String::new();

    io::stdin().read_line(&mut str).expect("Failed");
    println!("{}", str);
}
