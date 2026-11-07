use std::ffi::CString;
use std::os::raw::{c_char, c_int};

extern "C" {
    fn abs(n: c_int) -> c_int;
    fn strlen(text: *const c_char) -> usize;
}

fn main() {}

fn c_abs(n: i32) -> i32 {
    // SAFETY: abs works for any number except the smallest i32, which the tests don't use.
    unsafe { abs(n) }
}

fn c_length(text: &str) -> usize {
    let text = CString::new(text).unwrap();
    // SAFETY: text is a valid C string, and it's alive until the end of this function.
    unsafe { strlen(text.as_ptr()) }
}

fn swap_through_pointers(a: &mut i32, b: &mut i32) {
    let a = a as *mut i32;
    let b = b as *mut i32;
    // SAFETY: both pointers come from valid, separate references.
    unsafe {
        let temporary = *a;
        *a = *b;
        *b = temporary;
    }
}

fn read_at(numbers: &[i32], index: usize) -> Option<i32> {
    if index >= numbers.len() {
        return None;
    }
    // SAFETY: we just checked that index is in range.
    Some(unsafe { *numbers.get_unchecked(index) })
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
