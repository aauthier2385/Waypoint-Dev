use std::f64::consts::PI;

fn main() {
    let heading = Direction::North;
    println!("Turning right from {:?} faces {:?}", heading, heading.turn_right());
    println!("A quarter is worth {} cents", value_in_cents(&Coin::Quarter));
    println!("A circle with radius 2 has area {:.2}", area(&Shape::Circle(2.0)));
    println!("42 is {}", size_word(42));
}

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        todo!()
    }

    fn opposite(&self) -> Direction {
        todo!()
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: &Coin) -> u32 {
    todo!()
}

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle { base: f64, height: f64 },
}

fn area(shape: &Shape) -> f64 {
    todo!()
}

/// "negative", "zero", "small" (1-9), "medium" (10-99), or "large".
fn size_word(number: i32) -> String {
    todo!()
}

// ---------------------------------------------------------------
// The checker. You don't need to change anything below this line.
// ---------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turning() {
        assert_eq!(Direction::North.turn_right(), Direction::East);
        assert_eq!(Direction::East.turn_right(), Direction::South);
        assert_eq!(Direction::South.turn_right(), Direction::West);
        assert_eq!(Direction::West.turn_right(), Direction::North);
    }

    #[test]
    fn opposites() {
        assert_eq!(Direction::North.opposite(), Direction::South);
        assert_eq!(Direction::East.opposite(), Direction::West);
        assert_eq!(Direction::West.opposite(), Direction::East);
    }

    #[test]
    fn coins() {
        let total: u32 = [Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter].iter().map(value_in_cents).sum();
        assert_eq!(total, 41, "1 + 5 + 10 + 25 = 41");
    }

    #[test]
    fn areas() {
        assert!((area(&Shape::Circle(1.0)) - PI).abs() < 0.001, "a circle with radius 1 has area π");
        assert_eq!(area(&Shape::Rectangle(3.0, 4.0)), 12.0);
        assert_eq!(area(&Shape::Triangle { base: 6.0, height: 2.0 }), 6.0);
    }

    #[test]
    fn sizes() {
        assert_eq!(size_word(-5), "negative");
        assert_eq!(size_word(0), "zero");
        assert_eq!(size_word(9), "small");
        assert_eq!(size_word(10), "medium");
        assert_eq!(size_word(99), "medium");
        assert_eq!(size_word(100), "large");
    }
}
