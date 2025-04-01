use std::io;

fn main() {
    println!("Enter Your name:");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");

    let name = name.trim();

    println!("Hello, {}!", name);
}