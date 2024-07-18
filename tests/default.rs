use fizz_buzz_macro::fizz_buzz_generator;

#[test]
fn default_test() {
    let fizz_buzz = fizz_buzz_generator! {3 => "Fizz", 5 => "Buzz"};
    assert_eq!("1", fizz_buzz(1));
    assert_eq!("2", fizz_buzz(2));
    assert_eq!("Fizz", fizz_buzz(3));
    assert_eq!("4", fizz_buzz(4));
    assert_eq!("Buzz", fizz_buzz(5));
    assert_eq!("Fizz", fizz_buzz(6));
    assert_eq!("7", fizz_buzz(7));
    assert_eq!("8", fizz_buzz(8));
    assert_eq!("Fizz", fizz_buzz(9));
    assert_eq!("Buzz", fizz_buzz(10));
    assert_eq!("11", fizz_buzz(11));
    assert_eq!("Fizz", fizz_buzz(12));
    assert_eq!("13", fizz_buzz(13));
    assert_eq!("14", fizz_buzz(14));
    assert_eq!("FizzBuzz", fizz_buzz(15));
}

#[test]
fn different_test() {
    let fizz_buzz = fizz_buzz_generator! {3 => "Fizz", 4 => "Buzz", 7 => "Rain"};
    assert_eq!("1", fizz_buzz(1));
    assert_eq!("2", fizz_buzz(2));
    assert_eq!("Fizz", fizz_buzz(3));
    assert_eq!("Buzz", fizz_buzz(4));
    assert_eq!("5", fizz_buzz(5));
    assert_eq!("Fizz", fizz_buzz(6));
    assert_eq!("Rain", fizz_buzz(7));
    assert_eq!("Buzz", fizz_buzz(8));
    assert_eq!("Fizz", fizz_buzz(9));
    assert_eq!("10", fizz_buzz(10));
    assert_eq!("11", fizz_buzz(11));
    assert_eq!("FizzBuzz", fizz_buzz(12));
    assert_eq!("13", fizz_buzz(13));
    assert_eq!("Rain", fizz_buzz(14));
    assert_eq!("Fizz", fizz_buzz(15));
    assert_eq!("Buzz", fizz_buzz(16));
    assert_eq!("17", fizz_buzz(17));
    assert_eq!("Fizz", fizz_buzz(18));
    assert_eq!("19", fizz_buzz(19));
    assert_eq!("Buzz", fizz_buzz(20));
    assert_eq!("FizzRain", fizz_buzz(21));
}

#[test]
fn different_separator() {
    let fizz_buzz = fizz_buzz_generator! {3 => "Fizz", 5 => "Buzz", " - "};
    assert_eq!("1", fizz_buzz(1));
    assert_eq!("2", fizz_buzz(2));
    assert_eq!("Fizz", fizz_buzz(3));
    assert_eq!("4", fizz_buzz(4));
    assert_eq!("Buzz", fizz_buzz(5));
    assert_eq!("Fizz", fizz_buzz(6));
    assert_eq!("7", fizz_buzz(7));
    assert_eq!("8", fizz_buzz(8));
    assert_eq!("Fizz", fizz_buzz(9));
    assert_eq!("Buzz", fizz_buzz(10));
    assert_eq!("11", fizz_buzz(11));
    assert_eq!("Fizz", fizz_buzz(12));
    assert_eq!("13", fizz_buzz(13));
    assert_eq!("14", fizz_buzz(14));
    assert_eq!("Fizz - Buzz", fizz_buzz(15));
    assert_eq!("16", fizz_buzz(16));
}
