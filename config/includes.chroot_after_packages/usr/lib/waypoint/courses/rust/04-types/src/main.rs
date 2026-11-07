fn main() {
    // A whole number that can't be negative.
    let apples: u32 = 12;

    // A number with a decimal point.
    let price: u32 = 0.75;

    // true or false.
    let on_sale: bool = "yes";

    // One single character.
    let star: char = "★";

    // Text.
    let fruit: &str = "apple";

    let total = apples as f64 * price;
    println!("{apples} {fruit}s at ${price:.2} each costs ${total:.2}.");
    println!("On sale: {on_sale}");
    println!("Rating: {star}");
}
