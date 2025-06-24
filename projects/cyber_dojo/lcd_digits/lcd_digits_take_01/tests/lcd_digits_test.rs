use rstest::rstest;

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

const DIGITS_REPR_910: &str = "\
._. ... ._.\n\
|_| ..| |.|\n\
..| ..| |_|\n";

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

#[rstest]
#[case::digit_zero(0, DIGITS_REPR_0)]
#[case::digit_one(1, DIGITS_REPR_1)]
#[case::digit_two(2, DIGITS_REPR_2)]
#[case::digit_three(3, DIGITS_REPR_3)]
#[case::digit_four(4, DIGITS_REPR_4)]
#[case::digit_five(5, DIGITS_REPR_5)]
#[case::digit_six(6, DIGITS_REPR_6)]
#[case::digit_seven(7, DIGITS_REPR_7)]
#[case::digit_eight(8, DIGITS_REPR_8)]
#[case::digit_nine(9, DIGITS_REPR_9)]
#[case::all_digits(1234567890, DIGITS_REPR_1234567890)]
#[case::three_digits_910(910, DIGITS_REPR_910)]
fn test_generate_lcd_digits(#[case] input: i32, #[case] expected: &str) {
    let lcd = LcdDigits::new();
    let result = lcd.generate(input, None).unwrap();
    let result = replace_lcd_digit_cells(&result);
    let result = replace_lcd_digit_separators(&result, SEPARATOR, TEST_SEPARATOR);

    assert_eq!(
        result, expected,
        "test_generate_lcd_digits for {} = \n'{}'\nexpected:\n'{}'",
        input, result, expected
    );
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
