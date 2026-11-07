pub fn biggest(numbers: &[i32]) -> i32 {
    let mut best = numbers[0];
    for &number in numbers {
        if number > best {
            best = number;
        }
    }
    best
}

pub fn smallest(numbers: &[i32]) -> i32 {
    let mut best = numbers[0];
    for &number in numbers {
        if number < best {
            best = number;
        }
    }
    best
}
