use interviewer::{ask_many, set_consumable_quotes, Separator};

fn main() {
    // no escaping
    set_consumable_quotes(false);
    let s: Vec<String> = ask_many("Enter some strings: ", Separator::Whitespace).unwrap();
    // assume input was "a "b c d" e f"
    println!("{:?}", s); // ["a", "\"b", "c", "d\"", "e", "f"]

    // escaping
    set_consumable_quotes(true);
    let s: Vec<String> = ask_many("Enter some strings: ", Separator::Whitespace).unwrap();
    // assume input was "a "b c d" e f"
    println!("{:?}", s); // ["a", "b c d", "e", "f"]
}
