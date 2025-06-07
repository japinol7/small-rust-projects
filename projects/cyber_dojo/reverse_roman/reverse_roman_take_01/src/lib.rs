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

/// Converts a Roman numeral string to an integer
///
/// # Examples
///
/// ```
/// use reverse_roman::from_roman;
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
