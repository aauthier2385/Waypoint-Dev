use std::f64::consts::PI;
use std::fmt;

fn main() {
    let rex = Dog { name: "Rex".to_string() };
    let car = Car { make: "Ford".to_string(), year: 2020 };
    println!("{}", introduce(&rex));
    println!("{}", introduce(&car));
    println!("{}", rex.shout());
    println!("It's {} outside", Temperature { degrees: 21.5 });

    let square = Square { side: 2.0 };
    let circle = Circle { radius: 1.0 };
    println!("Total area: {:.2}", total_area(&[&square, &circle]));
}

trait Describe {
    fn describe(&self) -> String;

    fn shout(&self) -> String {
        self.describe().to_uppercase()
    }
}

struct Dog {
    name: String,
}

impl Describe for Dog {
    fn describe(&self) -> String {
        format!("a dog named {}", self.name)
    }
}

struct Car {
    make: String,
    year: u32,
}

// Give Car the Describe ability here.

/// "Meet " followed by the description.
fn introduce(thing: &impl Describe) -> String {
    todo!()
}

struct Temperature {
    degrees: f64,
}

// Give Temperature the fmt::Display ability here, so it prints like 21.5°C.

trait Area {
    fn area(&self) -> f64;
}

struct Square {
    side: f64,
}

struct Circle {
    radius: f64,
}

// Give Square and Circle the Area ability here.

/// Adds up the area of every shape.
fn total_area(shapes: &[&dyn Area]) -> f64 {
    todo!()
}

// ---------------------------------------------------------------
// The checker. You don't need to change anything below this line.
// ---------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describing() {
        let car = Car { make: "Ford".to_string(), year: 2020 };
        assert_eq!(car.describe(), "a 2020 Ford");
        assert_eq!(car.shout(), "A 2020 FORD", "shout comes free with Describe");
    }

    #[test]
    fn introducing() {
        let rex = Dog { name: "Rex".to_string() };
        let car = Car { make: "Kia".to_string(), year: 2015 };
        assert_eq!(introduce(&rex), "Meet a dog named Rex");
        assert_eq!(introduce(&car), "Meet a 2015 Kia");
    }

    #[test]
    fn displaying() {
        assert_eq!(Temperature { degrees: 21.5 }.to_string(), "21.5°C");
        assert_eq!(format!("{}", Temperature { degrees: -3.0 }), "-3°C");
    }

    #[test]
    fn areas() {
        let square = Square { side: 2.0 };
        let circle = Circle { radius: 1.0 };
        assert_eq!(square.area(), 4.0);
        assert!((total_area(&[&square, &circle]) - (4.0 + PI)).abs() < 0.0001);
        assert_eq!(total_area(&[]), 0.0);
    }
}
