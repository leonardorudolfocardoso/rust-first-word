use std::io::stdin;

fn main() {
    println!("First word");

    println!("Enter your input:");

    let mut buf = String::new();
    stdin().read_line(&mut buf)
        .expect("Not able to read input. ");
}