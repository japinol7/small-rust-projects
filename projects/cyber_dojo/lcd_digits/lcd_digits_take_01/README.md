# LCD Digits

LCD digits display generator.


## Description

This program creates an LCD string representation of an integer value using a 3x3 grid of space, underscore, and pipe characters for each digit.

Each digit is shown below (using a dot instead of a space):

```
._.   ...   ._.   ._.   ...   ._.   ._.   ._.   ._.   ._.
|.|   ..|   ._|   ._|   |_|   |_.   |_.   ..|   |_|   |_|
|_|   ..|   |_.   ._|   ..|   ._|   |_|   ..|   |_|   ..|
```

### Example: 910

```
._. ... ._.
|_| ..| |.|
..| ..| |_|
```

## Usage

To run the program:

```
cargo run
```

To run the tests:

```
cargo test
```

## Structure

- `src/lcd_digit_cell.rs`: Contains the cell constants and digit templates
- `src/lcd_digits.rs`: Contains the main LCD digit generation logic
- `src/main.rs`: Entry point for the application
- `tests/lcd_digits_test.rs`: Tests for the LCD digit generation
