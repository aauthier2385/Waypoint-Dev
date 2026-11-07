/// "A 3 by 3 square has an area of 9"
fn describe_square(size: u32) -> String {
    format!("A {size} by {size} square has an area of {}", square_area(size))
}

fn square_area(size: u32) -> u32 {
    size * size
}
