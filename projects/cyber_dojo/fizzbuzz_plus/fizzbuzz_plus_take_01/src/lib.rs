/// Checks if number n contains digit d
fn contains_digit(n: i32, d: i32) -> bool {
    n.to_string().contains(&d.to_string())
}

/// Converts a number to its FizzBuzz string representation
pub fn fizzbuzz(n: i32) -> String {
    let mut res = String::new();

    if n % 3 == 0 || contains_digit(n, 3) {
        res.push_str("Fizz");
    }
    if n % 5 == 0 || contains_digit(n, 5) {
        res.push_str("Buzz");
    }

    if res.is_empty() {
        return n.to_string();
    }
    res
}

/// Generates FizzBuzz strings for numbers from 1 to n
pub fn fizzbuzz_range(n: i32) -> Vec<String> {
    (1..=n).map(fizzbuzz).collect()
}

/// Converts a vector of strings to a newline-separated string
pub fn to_string(items: &[String]) -> String {
    items.join("\n")
}
