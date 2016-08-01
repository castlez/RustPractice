use std::io::{self, Write};

fn main() {
    print!("enter something >>");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input);

    println!("you entered: {}", input);
}
