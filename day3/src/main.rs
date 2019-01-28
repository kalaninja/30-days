use std::io;

fn main() {
    let mut str = String::new();

    io::stdin().read_line(&mut str).expect("Failed");

    let n = str.trim().parse::<u32>().expect("Failed");
    if n % 2 == 1 || (n >= 6 && n <= 20) {
        println!("Weird")
    } else {
        println!("Not Weird")
    }
}
