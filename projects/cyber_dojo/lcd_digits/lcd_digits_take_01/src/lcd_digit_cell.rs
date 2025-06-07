// Chars used to display LCD digits and the separator string
pub const CELL_H: char = '_'; // Horizontal segment
pub const CELL_V: char = '|'; // Vertical segment
pub const CELL_O: char = '.'; // Off segment
pub const SEPARATOR: &str = " ";

// Create a string from chars for a digit template with placeholders for separators
macro_rules! create_digit_template {
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $i:expr) => {
        format!("{}{}{}{}{}{}{}{}{}", $a, $b, $c, $d, $e, $f, $g, $h, $i)
    };
}

// Initialize the digit templates for each number
pub fn get_digit_template(digit: u32) -> &'static str {
    match digit {
        0 => {
            lazy_static::lazy_static! {
                static ref DIGIT: String = create_digit_template!(
                    CELL_O, CELL_H, CELL_O, CELL_V, CELL_O, CELL_V, CELL_V, CELL_H, CELL_V);
            }
            &DIGIT
        }
        1 => {
            lazy_static::lazy_static! {
                static ref DIGIT: String = create_digit_template!(
                    CELL_O, CELL_O, CELL_O, CELL_O, CELL_O, CELL_V, CELL_O, CELL_O, CELL_V);
            }
            &DIGIT
        }
        2 => {
            lazy_static::lazy_static! {
                static ref DIGIT: String = create_digit_template!(
                    CELL_O, CELL_H, CELL_O, CELL_O, CELL_H, CELL_V, CELL_V, CELL_H, CELL_O);
            }
            &DIGIT
        }
        3 => {
            lazy_static::lazy_static! {
                static ref DIGIT: String = create_digit_template!(
                    CELL_O, CELL_H, CELL_O, CELL_O, CELL_H, CELL_V, CELL_O, CELL_H, CELL_V);
            }
            &DIGIT
        }
        4 => {
            lazy_static::lazy_static! {
                static ref DIGIT: String = create_digit_template!(
                    CELL_O, CELL_O, CELL_O, CELL_V, CELL_H, CELL_V, CELL_O, CELL_O, CELL_V);
            }
            &DIGIT
        }
        5 => {
            lazy_static::lazy_static! {
                static ref DIGIT: String = create_digit_template!(
                    CELL_O, CELL_H, CELL_O, CELL_V, CELL_H, CELL_O, CELL_O, CELL_H, CELL_V);
            }
            &DIGIT
        }
        6 => {
            lazy_static::lazy_static! {
                static ref DIGIT: String = create_digit_template!(
                    CELL_O, CELL_H, CELL_O, CELL_V, CELL_H, CELL_O, CELL_V, CELL_H, CELL_V);
            }
            &DIGIT
        }
        7 => {
            lazy_static::lazy_static! {
                static ref DIGIT: String = create_digit_template!(
                    CELL_O, CELL_H, CELL_O, CELL_O, CELL_O, CELL_V, CELL_O, CELL_O, CELL_V);
            }
            &DIGIT
        }
        8 => {
            lazy_static::lazy_static! {
                static ref DIGIT: String = create_digit_template!(
                    CELL_O, CELL_H, CELL_O, CELL_V, CELL_H, CELL_V, CELL_V, CELL_H, CELL_V);
            }
            &DIGIT
        }
        9 => {
            lazy_static::lazy_static! {
                static ref DIGIT: String = create_digit_template!(
                    CELL_O, CELL_H, CELL_O, CELL_V, CELL_H, CELL_V, CELL_O, CELL_O, CELL_V);
            }
            &DIGIT
        }
        _ => panic!("Invalid digit: {}", digit),
    }
}
