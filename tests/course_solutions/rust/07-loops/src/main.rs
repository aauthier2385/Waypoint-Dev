fn main() {}

fn sum_up_to(n: u32) -> u32 {
    let mut total = 0;
    for number in 1..=n {
        total += number;
    }
    total
}

fn count_vowels(text: &str) -> u32 {
    let mut count = 0;
    for letter in text.chars() {
        if "aeiouAEIOU".contains(letter) {
            count += 1;
        }
    }
    count
}

fn doublings_to_reach(start: u32, goal: u32) -> u32 {
    let mut amount = start;
    let mut doublings = 0;
    while amount < goal {
        amount *= 2;
        doublings += 1;
    }
    doublings
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
