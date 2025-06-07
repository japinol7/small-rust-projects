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
/// use roman_numerals_full::to_roman;
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

/// Converts a Roman numeral string to an integer
///
/// # Examples
///
/// ```
/// use roman_numerals_full::from_roman;
///
/// assert_eq!(from_roman("XLII"), Ok(42));
/// ```
pub fn from_roman(roman: &str) -> Result<i32, RomanError> {
    if roman.is_empty() {
        return Err(RomanError::EmptyString);
    }

    // Map for Roman numeral symbols to values
    let roman_values = [
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ];
    let roman_map: std::collections::HashMap<char, i32> = roman_values.into_iter().collect();

    let mut result = 0;
    let mut prev_value = 0;

    // Process from right to left
    for c in roman.chars().rev() {
        match roman_map.get(&c) {
            Some(&value) => {
                if value >= prev_value {
                    result += value;
                } else {
                    result -= value;
                }
                prev_value = value;
            }
            None => return Err(RomanError::InvalidCharacter(c)),
        }
    }

    // Check if the result is within the valid range for Roman numerals
    if result <= 0 || result > 3999 {
        return Err(RomanError::OutOfRange);
    }

    Ok(result)
}
