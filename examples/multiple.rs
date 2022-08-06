use interviewer::{ask_many, Separator};

fn main() {
    let floats: Vec<f64> = ask_many("Enter your name: ", Separator::Sequence(",")).unwrap();
    // input elements need to be separated by "," in order to be parsed correctly
    println!("{:?}", floats); // [1.0, 2.0, 3.0]
}
