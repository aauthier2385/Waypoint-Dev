use std::f64::consts::PI;
use std::fmt;

fn main() {}

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

impl Describe for Car {
    fn describe(&self) -> String {
        format!("a {} {}", self.year, self.make)
    }
}

fn introduce(thing: &impl Describe) -> String {
    format!("Meet {}", thing.describe())
}

struct Temperature {
    degrees: f64,
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}°C", self.degrees)
    }
}

trait Area {
    fn area(&self) -> f64;
}

struct Square {
    side: f64,
}

struct Circle {
    radius: f64,
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

fn total_area(shapes: &[&dyn Area]) -> f64 {
    let mut total = 0.0;
    for shape in shapes {
        total += shape.area();
    }
    total
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
