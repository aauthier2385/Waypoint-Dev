fn main() {
    // An example: count down from 3.
    for number in (1..=3).rev() {
        println!("{number}...");
    }
    println!("Liftoff!");

    println!("1 + 2 + ... + 10 = {}", sum_up_to(10));
    println!("\"programming\" has {} vowels", count_vowels("programming"));
    println!("Doubling 1 reaches 100 after {} doublings", doublings_to_reach(1, 100));
}

/// Adds up every whole number from 1 to n. sum_up_to(3) is 1 + 2 + 3 = 6.
fn sum_up_to(n: u32) -> u32 {
    todo!()
}

/// Counts the vowels (a, e, i, o, u), uppercase or lowercase.
fn count_vowels(text: &str) -> u32 {
    todo!()
}

/// How many times do you double start before it reaches goal or more?
fn doublings_to_reach(start: u32, goal: u32) -> u32 {
    todo!()
}

// ---------------------------------------------------------------
// The checker. You don't need to change anything below this line.
// ---------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums() {
        assert_eq!(sum_up_to(3), 6, "sum_up_to(3) should be 1 + 2 + 3 = 6");
        assert_eq!(sum_up_to(10), 55, "sum_up_to(10) should be 55");
        assert_eq!(sum_up_to(0), 0, "sum_up_to(0) should be 0");
    }

    #[test]
    fn vowels() {
        assert_eq!(count_vowels("rust"), 1, "\"rust\" has 1 vowel");
        assert_eq!(count_vowels("programming"), 3, "\"programming\" has 3 vowels");
        assert_eq!(count_vowels("Waypoint OS"), 4, "\"Waypoint OS\" has 4 vowels, counting capitals");
        assert_eq!(count_vowels(""), 0, "empty text has 0 vowels");
    }

    #[test]
    fn doublings() {
        assert_eq!(doublings_to_reach(1, 8), 3, "1 doubles to 2, 4, 8: that is 3 doublings");
        assert_eq!(doublings_to_reach(5, 5), 0, "5 already reaches 5, so 0 doublings");
        assert_eq!(doublings_to_reach(3, 20), 3, "3 doubles to 6, 12, 24: that is 3 doublings");
    }
}
