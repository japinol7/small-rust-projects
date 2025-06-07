# Roman Numerals Full

A Rust implementation of Roman numeral conversion functions.

## Features

- Convert integers to Roman numerals (`to_roman` function)
- Convert Roman numerals to integers (`from_roman` function)
- Test suite

## Usage

### Convert integers to Roman numerals

```rust
use roman_numerals_full::to_roman;

fn main() {
    match to_roman(42) {
        Ok(roman) => println!("42 in Roman numerals is {}", roman),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Convert Roman numerals to integers

```rust
use roman_numerals_full::from_roman;

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
