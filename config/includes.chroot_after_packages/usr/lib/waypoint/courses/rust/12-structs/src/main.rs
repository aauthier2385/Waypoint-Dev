fn main() {
    let door = Rectangle::new(3, 7);
    let tile = Rectangle::square(2);
    println!("{door:?} has an area of {}", door.area());
    println!("Can the door hold the tile? {}", door.can_hold(&tile));

    let mut account = BankAccount::new("Sam");
    account.deposit(50);
    println!("Withdrew 80? {}", account.withdraw(80));
    println!("{account:?}");
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    /// Finished, as an example.
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    /// A rectangle with the same width and height.
    fn square(size: u32) -> Rectangle {
        todo!()
    }

    fn area(&self) -> u32 {
        todo!()
    }

    /// The distance all the way around.
    fn perimeter(&self) -> u32 {
        todo!()
    }

    /// True if other fits completely inside this rectangle.
    fn can_hold(&self, other: &Rectangle) -> bool {
        todo!()
    }
}

#[derive(Debug)]
struct BankAccount {
    owner: String,
    balance: u32,
}

impl BankAccount {
    /// A new account with a balance of 0.
    fn new(owner: &str) -> BankAccount {
        todo!()
    }

    fn deposit(&mut self, amount: u32) {
        todo!()
    }

    /// Takes money out and gives back true. If there isn't enough, changes nothing and gives back false.
    fn withdraw(&mut self, amount: u32) -> bool {
        todo!()
    }
}

// ---------------------------------------------------------------
// The checker. You don't need to change anything below this line.
// ---------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn squares() {
        let tile = Rectangle::square(4);
        assert_eq!((tile.width, tile.height), (4, 4));
    }

    #[test]
    fn measuring() {
        let door = Rectangle::new(3, 7);
        assert_eq!(door.area(), 21);
        assert_eq!(door.perimeter(), 20);
    }

    #[test]
    fn fitting() {
        let big = Rectangle::new(10, 8);
        assert!(big.can_hold(&Rectangle::new(5, 5)));
        assert!(!big.can_hold(&Rectangle::new(11, 2)), "too wide to fit");
        assert!(!Rectangle::new(5, 5).can_hold(&big), "a small one can't hold a big one");
    }

    #[test]
    fn banking() {
        let mut account = BankAccount::new("Sam");
        assert_eq!(account.owner, "Sam");
        assert_eq!(account.balance, 0, "new accounts start at 0");
        account.deposit(50);
        assert!(account.withdraw(20), "there's enough for 20");
        assert_eq!(account.balance, 30);
        assert!(!account.withdraw(100), "there isn't enough for 100");
        assert_eq!(account.balance, 30, "a failed withdrawal changes nothing");
    }
}
