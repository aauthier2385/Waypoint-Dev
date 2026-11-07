/// The biggest number in the list. Finished, as an example.
pub fn biggest(numbers: &[i32]) -> i32 {
    let mut best = numbers[0];
    for &number in numbers {
        if number > best {
            best = number;
        }
    }
    best
}

/// The smallest number in the list.
pub fn smallest(numbers: &[i32]) -> i32 {
    todo!()
}
