use crate::lcd_digit_cell::{SEPARATOR, get_digit_template};
use std::fmt;

#[derive(Debug)]
pub struct LcdDigitsError {
    message: String,
}

impl fmt::Display for LcdDigitsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for LcdDigitsError {}

pub struct LcdDigits;

impl LcdDigits {
    pub fn new() -> Self {
        LcdDigits
    }

    /// Generate creates an LCD representation of a number
    pub fn generate(&self, num: i32, separator: Option<&str>) -> Result<String, LcdDigitsError> {
        let sep = separator.unwrap_or(SEPARATOR);

        if num < 0 {
            return Err(LcdDigitsError {
                message: "Number must be non-negative".to_string(),
            });
        }

        // Handle the single digit case
        if num <= 9 {
            let digit_template = get_digit_template(num as u32);
            // Split the template into three parts and add newlines between them
            return Ok(format!(
                "{}\n{}\n{}\n",
                &digit_template[..3],
                &digit_template[3..6],
                &digit_template[6..9]
            ));
        }

        // Handle the multi-digit case
        match self.generate_multiple_digits(num, sep) {
            Ok(digits) => Ok(digits.join("")),
            Err(e) => Err(e),
        }
    }

    /// Generate LCD representations for multi-digit numbers
    fn generate_multiple_digits(
        &self,
        num: i32,
        separator: &str,
    ) -> Result<Vec<String>, LcdDigitsError> {
        let num_str = num.to_string();
        let num_len = num_str.len();

        let mut digits_top = Vec::with_capacity(num_len);
        let mut digits_mid = Vec::with_capacity(num_len);
        let mut digits_bottom = Vec::with_capacity(num_len);

        for (i, digit_char) in num_str.chars().enumerate() {
            let digit = digit_char.to_digit(10).ok_or_else(|| LcdDigitsError {
                message: "Invalid digit".to_string(),
            })?;

            let cur_separator = if i < num_len - 1 { separator } else { "\n" };

            let digit_template = get_digit_template(digit);

            // Split the digit template into 3 parts (each part is 3 chars long)
            let top_part = format!("{}{}", &digit_template[..3], cur_separator);
            let mid_part = format!("{}{}", &digit_template[3..6], cur_separator);
            let bottom_part = format!("{}{}", &digit_template[6..9], cur_separator);

            digits_top.push(top_part);
            digits_mid.push(mid_part);
            digits_bottom.push(bottom_part);
        }

        // Combine all parts
        let mut result =
            Vec::with_capacity(digits_top.len() + digits_mid.len() + digits_bottom.len());
        result.extend(digits_top);
        result.extend(digits_mid);
        result.extend(digits_bottom);

        Ok(result)
    }
}
