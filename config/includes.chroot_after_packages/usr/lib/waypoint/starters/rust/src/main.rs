// Prints a times table. Try changing the number below.
fn main() {
    let number = 7;
    for multiplier in 1..=10 {
        println!("{number} x {multiplier} = {}", number * multiplier);
    }
}
