fn main() {}

fn squares_of_evens(numbers: &[i32]) -> Vec<i32> {
    numbers.iter().filter(|n| *n % 2 == 0).map(|n| n * n).collect()
}

fn total_with_tax(prices: &[f64], tax_rate: f64) -> f64 {
    prices.iter().map(|price| price * (1.0 + tax_rate)).sum()
}

fn names_starting_with(names: &[&str], letter: char) -> Vec<String> {
    names.iter().filter(|name| name.starts_with(letter)).map(|name| name.to_string()).collect()
}

fn count_long_words(text: &str, min_length: usize) -> usize {
    text.split_whitespace().filter(|word| word.len() >= min_length).count()
}

fn apply_twice(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}

fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}

// ---------------------------------------------------------------
// The checker. You don't need to change anything below this line.
// ---------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evens_squared() {
        assert_eq!(squares_of_evens(&[1, 2, 3, 4]), vec![4, 16]);
        assert_eq!(squares_of_evens(&[1, 3]), Vec::<i32>::new());
    }

    #[test]
    fn tax() {
        assert!((total_with_tax(&[10.0, 20.0], 0.10) - 33.0).abs() < 0.0001, "30 plus 10% tax is 33");
        assert_eq!(total_with_tax(&[], 0.10), 0.0);
    }

    #[test]
    fn names() {
        assert_eq!(names_starting_with(&["Sam", "Ana", "Sid"], 'S'), vec!["Sam", "Sid"]);
        assert!(names_starting_with(&["Ana"], 'Z').is_empty());
    }

    #[test]
    fn long_words() {
        assert_eq!(count_long_words("a quick brown fox", 4), 2);
        assert_eq!(count_long_words("", 1), 0);
    }

    #[test]
    fn twice() {
        assert_eq!(apply_twice(|n| n + 3, 10), 16);
        assert_eq!(apply_twice(|n| n * n, 3), 81);
    }

    #[test]
    fn multipliers() {
        let triple = make_multiplier(3);
        let negate = make_multiplier(-1);
        assert_eq!(triple(7), 21);
        assert_eq!(negate(5), -5);
    }
}
