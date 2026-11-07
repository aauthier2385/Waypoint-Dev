use grading::{fizzbuzz, is_leap_year, letter_grade};

#[test]
fn fizz_buzz_and_both() {
    assert_eq!(fizzbuzz(3), "Fizz");
    assert_eq!(fizzbuzz(5), "Buzz");
    assert_eq!(fizzbuzz(15), "FizzBuzz");
}

#[test]
fn plain_numbers() {
    assert_eq!(fizzbuzz(7), "7");
}

#[test]
fn centuries_are_not_leap_years() {
    assert!(!is_leap_year(1900));
}

#[test]
fn every_400_years_is_a_leap_year() {
    assert!(is_leap_year(2000));
    assert!(is_leap_year(2024));
}

#[test]
fn exactly_ninety_is_an_a() {
    assert_eq!(letter_grade(90), 'A');
    assert_eq!(letter_grade(59), 'F');
}
