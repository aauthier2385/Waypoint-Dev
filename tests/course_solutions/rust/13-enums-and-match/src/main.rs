use std::f64::consts::PI;

fn main() {}

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle { base: f64, height: f64 },
}

fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => PI * radius * radius,
        Shape::Rectangle(width, height) => width * height,
        Shape::Triangle { base, height } => base * height / 2.0,
    }
}

fn size_word(number: i32) -> String {
    match number {
        i32::MIN..=-1 => "negative".to_string(),
        0 => "zero".to_string(),
        1..=9 => "small".to_string(),
        10..=99 => "medium".to_string(),
        _ => "large".to_string(),
    }
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
