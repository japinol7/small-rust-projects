//! Roman numeral conversion library

/// Error type for Roman numeral operations
#[derive(Debug, PartialEq)]
pub enum RomanError {
    /// The number is out of the valid range (1-3999)
    OutOfRange,
    /// The Roman numeral string is empty
    EmptyString,
    /// The Roman numeral string contains invalid characters
    InvalidCharacter(char),
}

impl std::fmt::Display for RomanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RomanError::OutOfRange => write!(f, "number out of range (1-3999)"),
            RomanError::EmptyString => write!(f, "empty Roman numeral string"),
            RomanError::InvalidCharacter(c) => write!(f, "invalid Roman numeral character: {}", c),
        }
    }
}

impl std::error::Error for RomanError {}

/// Converts an integer to a Roman numeral string
///
/// # Examples
///
/// ```
/// use roman_numerals::to_roman;
///
/// assert_eq!(to_roman(42), Ok("XLII".to_string()));
/// ```
pub fn to_roman(num: i32) -> Result<String, RomanError> {
    if num <= 0 || num > 3999 {
        return Err(RomanError::OutOfRange);
    }

    // Define Roman numeral symbols and their values
    let values = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
    let numerals = [
        "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
    ];

    let mut result = String::new();
    let mut num_remaining = num;

    for i in 0..values.len() {
        while num_remaining >= values[i] {
            result.push_str(numerals[i]);
            num_remaining -= values[i];
        }
    }

    Ok(result)
}
