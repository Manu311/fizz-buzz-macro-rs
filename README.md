# Fizz-Buzz-Generator macro function

Generates an efficient fizz-buzz-function depending on the provided parameters.

The default fizz-buzz-experience can be had with the following code:

```rust
use fizz_buzz_macro::fizz_buzz_generator;

let fizz_buzz = fizz_buzz_generator! {3 => "Fizz", 5 => "Buzz"};
assert_eq!("Fizz", fizz_buzz(3));
assert_eq!("Buzz", fizz_buzz(5));
assert_eq!("7", fizz_buzz(7));
assert_eq!("FizzBuzz", fizz_buzz(15));
```

Any number of replacements can be added like the two defaults.
Additionally it's also possible to define a glue string which will be used
in between the matches if multiple match.
For example:

```rust
use fizz_buzz_macro::fizz_buzz_generator;

let fizz_buzz = fizz_buzz_generator! {2 => "Even", 3 => "ByThree", " and "};
assert_eq!("Even", fizz_buzz(2));
assert_eq!("ByThree", fizz_buzz(3));
assert_eq!("Even and ByThree", fizz_buzz(6));
```
