use interviewer::{ask, Askable, Result};

#[derive(Debug)]
struct Single(f64);

impl Askable for Single {
    fn convert<S: AsRef<str>>(s: S) -> Result<Self>
    where Self: Sized {
        Ok(Single(s.as_ref().parse::<f64>().unwrap_or(4.2)))
    }
}

fn main() {
    let one: Single = ask("Enter a value: ").unwrap();
    println!("{:?}", one);
}
