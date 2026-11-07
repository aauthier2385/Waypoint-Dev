pub fn describe_square(size: u32) -> String {
    format!("A {size} by {size} square has an area of {}", square_area(size))
}

pub fn square_area(size: u32) -> u32 {
    size * size
}
