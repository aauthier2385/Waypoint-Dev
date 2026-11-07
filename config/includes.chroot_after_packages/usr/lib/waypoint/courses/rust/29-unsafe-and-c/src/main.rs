use std::ffi::CString;
use std::os::raw::{c_char, c_int};

// Functions from the C standard library, which every Linux program can use.
extern "C" {
    fn abs(n: c_int) -> c_int;
    fn strlen(text: *const c_char) -> usize;
}

fn main() {
    println!("{}", c_abs(-7));
    println!("{}", c_length("Waypoint"));

    let (mut a, mut b) = (1, 2);
    swap_through_pointers(&mut a, &mut b);
    println!("a = {a}, b = {b}");

    println!("{:?}", read_at(&[10, 20, 30], 1));
}

/// Uses C's abs. This doesn't build yet: read the error.
fn c_abs(n: i32) -> i32 {
    abs(n)
}

/// Counts the letters using C's strlen.
fn c_length(text: &str) -> usize {
    todo!()
}

/// Swaps two numbers, using raw pointers.
fn swap_through_pointers(a: &mut i32, b: &mut i32) {
    todo!()
}

/// The number at index, or None if there isn't one. Use get_unchecked safely.
fn read_at(numbers: &[i32], index: usize) -> Option<i32> {
    todo!()
}

// ---------------------------------------------------------------
// The checker. You don't need to change anything below this line.
// ---------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absolute_values() {
        assert_eq!(c_abs(-7), 7);
        assert_eq!(c_abs(12), 12);
    }

    #[test]
    fn c_strings() {
        assert_eq!(c_length("Waypoint"), 8);
        assert_eq!(c_length(""), 0);
    }

    #[test]
    fn swapping() {
        let (mut a, mut b) = (1, 2);
        swap_through_pointers(&mut a, &mut b);
        assert_eq!((a, b), (2, 1));
    }

    #[test]
    fn safe_wrapper() {
        assert_eq!(read_at(&[10, 20, 30], 1), Some(20));
        assert_eq!(read_at(&[10, 20, 30], 3), None, "position 3 doesn't exist in a list of 3");
        assert_eq!(read_at(&[], 0), None);
    }
}
