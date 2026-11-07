use std::collections::HashMap;

fn main() {
    println!("{:?}", average(&[2.0, 4.0, 9.0]));
    println!("{:?}", evens(&[1, 2, 3, 4, 5, 6]));
    println!("{:?}", remove_duplicates(&[3, 1, 3, 2, 1]));
    println!("{:?}", word_counts("the cat and THE hat"));

    let mut inventory = HashMap::new();
    add_stock(&mut inventory, "apples", 5);
    add_stock(&mut inventory, "apples", 3);
    println!("{inventory:?}");
}

/// The average, or None for an empty list.
fn average(numbers: &[f64]) -> Option<f64> {
    todo!()
}

/// Only the even numbers, in the same order.
fn evens(numbers: &[i32]) -> Vec<i32> {
    todo!()
}

/// Each number once, keeping the first time it appears.
fn remove_duplicates(numbers: &[i32]) -> Vec<i32> {
    todo!()
}

/// How many times each word appears, in lowercase.
fn word_counts(text: &str) -> HashMap<String, usize> {
    todo!()
}

/// Adds amount to the item, starting at 0 if it's new.
fn add_stock(inventory: &mut HashMap<String, u32>, item: &str, amount: u32) {
    todo!()
}

// ---------------------------------------------------------------
// The checker. You don't need to change anything below this line.
// ---------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn averages() {
        assert_eq!(average(&[2.0, 4.0, 9.0]), Some(5.0));
        assert_eq!(average(&[]), None, "an empty list has no average");
    }

    #[test]
    fn even_numbers() {
        assert_eq!(evens(&[1, 2, 3, 4, 5, 6]), vec![2, 4, 6]);
        assert_eq!(evens(&[1, 3]), Vec::<i32>::new());
    }

    #[test]
    fn duplicates() {
        assert_eq!(remove_duplicates(&[3, 1, 3, 2, 1]), vec![3, 1, 2]);
        assert_eq!(remove_duplicates(&[]), Vec::<i32>::new());
    }

    #[test]
    fn counting_words() {
        let counts = word_counts("the cat and THE hat");
        assert_eq!(counts.get("the"), Some(&2), "\"the\" appears twice, ignoring capitals");
        assert_eq!(counts.get("cat"), Some(&1));
        assert_eq!(counts.get("dog"), None);
        assert_eq!(counts.len(), 4, "four different words");
    }

    #[test]
    fn stocking() {
        let mut inventory = HashMap::new();
        add_stock(&mut inventory, "apples", 5);
        add_stock(&mut inventory, "apples", 3);
        add_stock(&mut inventory, "pears", 2);
        assert_eq!(inventory.get("apples"), Some(&8));
        assert_eq!(inventory.get("pears"), Some(&2));
    }
}
