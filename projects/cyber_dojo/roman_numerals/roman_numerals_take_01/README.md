# Roman Numerals

Given a positive integer number (eg 42) determine its Roman numeral
representation as a String (eg "XLII").

## Features

- Convert integers to Roman numerals (`to_roman` function)
- Test suite

## Usage

### Convert integers to Roman numerals

```rust
use roman_numerals::to_roman;

fn main() {
    match to_roman(42) {
        Ok(roman) => println!("42 in Roman numerals is {}", roman),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

## Limitations

This implementation supports Roman numerals in the range 1–3999, which covers the traditional Roman numeral system.

## Running Tests

```bash
cargo test
```
