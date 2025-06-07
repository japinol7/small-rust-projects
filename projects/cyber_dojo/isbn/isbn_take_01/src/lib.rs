//! ISBN - International Standard Book Number validation library
//!
//! This library provides functionality to validate ISBN-10 and ISBN-13 codes.

/// Constant representing the character 'X' in ISBN-10 calculations
const X_CHAR_INT_VALUE: i32 = -1;

/// Validates if a given string is a valid ISBN-13 code
fn validate_isbn13(isbn_digits: &[i32], isbn_dc: i32) -> bool {
    // Multiply each digit alternately by 1 or 3 and sum the results
    let mut check_digit = 0;
    for (i, &x) in isbn_digits.iter().enumerate() {
        if (i + 1) % 2 == 0 {
            check_digit += x * 3;
        } else {
            check_digit += x;
        }
    }
    check_digit %= 10;
    check_digit = (10 - check_digit) % 10;

    check_digit == isbn_dc
}

/// Validates if a given string is a valid ISBN-10 code
fn validate_isbn10(isbn_digits: &[i32], mut isbn_dc: i32) -> bool {
    if isbn_dc == X_CHAR_INT_VALUE {
        isbn_dc = 10;
    }

    // Multiply each digit by its position and sum the results
    let mut check_digit = 0;
    for (i, &x) in isbn_digits.iter().enumerate() {
        check_digit += x * (i as i32 + 1);
    }
    check_digit %= 11;

    check_digit == isbn_dc
}

/// Validates if a given string is a valid ISBN-13 or ISBN-10 code
///
/// Returns a boolean indicating whether the ISBN is valid.
///
/// # Examples
///
/// ```
/// use isbn::validate_isbn;
///
/// assert_eq!(validate_isbn("978-0-262-13472-9"), true); // Valid ISBN-13
/// assert_eq!(validate_isbn("0-8044-2957-X"), true);     // Valid ISBN-10
/// assert_eq!(validate_isbn("978-0-262-13472-1"), false); // Invalid ISBN-13
/// ```
pub fn validate_isbn(isbn: &str) -> bool {
    if isbn.is_empty() {
        return false;
    }

    // Check if all chars except the last one are numeric, '-', or space
    let chars: Vec<char> = isbn.chars().collect();
    for i in 0..chars.len() - 1 {
        let ch = chars[i];
        if !ch.is_ascii_digit() && ch != '-' && ch != ' ' {
            return false;
        }
    }

    // Extract digits for validation.
    // If the char is an X, extract it according to X_CHAR_INT_VALUE
    let mut isbn_nums = Vec::new();
    for ch in chars {
        if ch.is_ascii_digit() {
            let digit = ch.to_digit(10).unwrap() as i32;
            isbn_nums.push(digit);
        } else if ch == 'X' {
            isbn_nums.push(X_CHAR_INT_VALUE);
        }
    }

    let isbn_len = isbn_nums.len();
    if isbn_len != 13 && isbn_len != 10 {
        return false;
    }

    let isbn_nums_without_dc = &isbn_nums[..isbn_len - 1];
    let isbn_dc = isbn_nums[isbn_len - 1];

    if isbn_len == 13 {
        validate_isbn13(isbn_nums_without_dc, isbn_dc)
    } else if isbn_len == 10 {
        validate_isbn10(isbn_nums_without_dc, isbn_dc)
    } else {
        false
    }
}
