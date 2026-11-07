fn main() {}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[derive(Debug)]
struct BankAccount {
    owner: String,
    balance: u32,
}

impl BankAccount {
    fn new(owner: &str) -> BankAccount {
        BankAccount { owner: owner.to_string(), balance: 0 }
    }

    fn deposit(&mut self, amount: u32) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: u32) -> bool {
        if amount > self.balance {
            return false;
        }
        self.balance -= amount;
        true
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
