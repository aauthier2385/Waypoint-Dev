fn main() {
    let apples: u32 = 12;
    let price: f64 = 0.75;
    let on_sale: bool = true;
    let star: char = '★';
    let fruit: &str = "apple";

    let total = apples as f64 * price;
    println!("{apples} {fruit}s at ${price:.2} each costs ${total:.2}.");
    println!("On sale: {on_sale}");
    println!("Rating: {star}");
}
