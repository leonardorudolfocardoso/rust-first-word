use std::io::stdin;

fn main() {
    println!("First word");

    println!("Enter your input:");

    let mut buf = String::new();
    stdin().read_line(&mut buf)
        .expect("Not able to read input. ");


    println!("First word is \"{}\"", first_word(&buf));
}

fn first_word (s: &String) -> &str {
    match s.split_whitespace().next() {
        Some(word) => word,
        None => s
    }
}