use interviewer::{ask, ask_until};

fn main() {
    let name: String = ask_until("Enter your name: ");
    let age: u8 = ask("Enter your age: ").unwrap_or(0);
    println!("Hello, {}! You are {} years old.", name, age);
}
