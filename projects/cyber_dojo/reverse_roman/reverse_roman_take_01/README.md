# Reverse Roman

Given a Roman number as a string (eg "XX") determine its integer value (eg 20).

## Features

- Convert Roman numerals to integers (`from_roman` function)
- Test suite

## Usage

### Convert Roman numerals to integers

```rust
use reverse_roman::from_roman;

fn main() {
    match from_roman("XLII") {
        Ok(num) => println!("XLII in decimal is {}", num),
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
