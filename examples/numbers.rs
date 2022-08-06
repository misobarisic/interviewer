use interviewer::ask;

fn main() {
    let int16: i16 = ask("Enter an i16: ").unwrap_or(0);
    let int32: i32 = ask("Enter an i32: ").unwrap_or(0);
    let int64: i64 = ask("Enter an i64: ").unwrap_or(0);
    let int128: i128 = ask("Enter an i128: ").unwrap_or(0);
    let intptr: isize = ask("Enter an isize: ").unwrap_or(0);
    println!("{} {} {} {} {}", int16, int32, int64, int128, intptr);

    let uint16: u16 = ask("Enter a u16: ").unwrap_or(0);
    let uint32: u32 = ask("Enter a u32: ").unwrap_or(0);
    let uint64: u64 = ask("Enter a u64: ").unwrap_or(0);
    let uint128: u128 = ask("Enter a u128: ").unwrap_or(0);
    let uintptr: usize = ask("Enter a usize: ").unwrap_or(0);
    println!("{} {} {} {} {}", uint16, uint32, uint64, uint128, uintptr);

    // same as previous but for f32 and f64
    let float32: f32 = ask("Enter an f32: ").unwrap_or(0.0);
    let float64: f64 = ask("Enter an f64: ").unwrap_or(0.0);
    println!("{} {}", float32, float64);
}
