use weather::weather_report;

fn main() {
    println!("Water boils at {}°F", helpers::celsius_to_fahrenheit(100.0));
    for celsius in [-5.0, 21.0, 30.0] {
        println!("{}", weather_report(celsius));
    }
}
