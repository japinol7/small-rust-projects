use lcd_digits::LcdDigits;
use lcd_digits::lcd_digit_cell::{CELL_H, CELL_O, CELL_V, SEPARATOR};

// Chars used to display test LCD digits and the separator strings
const TEST_CELL_H: char = '_';
const TEST_CELL_V: char = '|';
const TEST_CELL_O: char = '.';
const TEST_SEPARATOR: &str = " ";
const TEST_SEPARATOR_BIG: &str = "     ";

// Test digit representations
const DIGITS_REPR_0: &str = "._.\n|.|\n|_|\n";
const DIGITS_REPR_1: &str = "...\n..|\n..|\n";
const DIGITS_REPR_2: &str = "._.\n._|\n|_.\n";
const DIGITS_REPR_3: &str = "._.\n._|\n._|\n";
const DIGITS_REPR_4: &str = "...\n|_|\n..|\n";
const DIGITS_REPR_5: &str = "._.\n|_.\n._|\n";
const DIGITS_REPR_6: &str = "._.\n|_.\n|_|\n";
const DIGITS_REPR_7: &str = "._.\n..|\n..|\n";
const DIGITS_REPR_8: &str = "._.\n|_|\n|_|\n";
const DIGITS_REPR_9: &str = "._.\n|_|\n..|\n";

const DIGITS_REPR_1234567890: &str = "\
... ._. ._. ... ._. ._. ._. ._. ._. ._.\n\
..| ._| ._| |_| |_. |_. ..| |_| |_| |.|\n\
..| |_. ._| ..| ._| |_| ..| |_| ..| |_|\n";

const DIGITS_REPR_1234567890_SEPARATOR_BIG: &str = "\
...     ._.     ._.     ...     ._.     ._.     ._.     ._.     ._.     ._.\n\
..|     ._|     ._|     |_|     |_.     |_.     ..|     |_|     |_|     |.|\n\
..|     |_.     ._|     ..|     ._|     |_|     ..|     |_|     ..|     |_|\n";

const DIGITS_REPR_910: &str = "._. ... ._.\n|_| ..| |.|\n..| ..| |_|\n";

// Replace LCD digit cells with test cells
fn replace_lcd_digit_cells(digit_cell_str: &str) -> String {
    let mut result = digit_cell_str.to_string();
    result = result.replace(CELL_H, &TEST_CELL_H.to_string());
    result = result.replace(CELL_V, &TEST_CELL_V.to_string());
    result = result.replace(CELL_O, &TEST_CELL_O.to_string());
    result
}

// Replace LCD digit separators with test separators
fn replace_lcd_digit_separators(
    digit_cell_str: &str,
    separator: &str,
    test_separator: &str,
) -> String {
    digit_cell_str.replace(separator, test_separator)
}

#[test]
fn test_generate_lcd_digits() {
    let tests = vec![
        ("0", 0, DIGITS_REPR_0),
        ("1", 1, DIGITS_REPR_1),
        ("2", 2, DIGITS_REPR_2),
        ("3", 3, DIGITS_REPR_3),
        ("4", 4, DIGITS_REPR_4),
        ("5", 5, DIGITS_REPR_5),
        ("6", 6, DIGITS_REPR_6),
        ("7", 7, DIGITS_REPR_7),
        ("8", 8, DIGITS_REPR_8),
        ("9", 9, DIGITS_REPR_9),
        ("all digits", 1234567890, DIGITS_REPR_1234567890),
        ("910", 910, DIGITS_REPR_910),
    ];

    let lcd = LcdDigits::new();

    for (name, input, expected) in tests {
        let result = lcd.generate(input, None).unwrap();
        let result = replace_lcd_digit_cells(&result);
        let result = replace_lcd_digit_separators(&result, SEPARATOR, TEST_SEPARATOR);

        assert_eq!(result, expected, "Test case '{}' failed", name);
    }
}

#[test]
fn test_generate_lcd_digits_with_custom_separator() {
    let lcd = LcdDigits::new();
    let separator = "    ";

    let result = lcd.generate(1234567890, Some(separator)).unwrap();
    let result = replace_lcd_digit_cells(&result);
    let result = replace_lcd_digit_separators(&result, separator, TEST_SEPARATOR_BIG);

    assert_eq!(result, DIGITS_REPR_1234567890_SEPARATOR_BIG);
}

#[test]
fn test_negative_num_should_return_error() {
    let lcd = LcdDigits::new();
    let result = lcd.generate(-1, None);
    assert!(result.is_err());
}
