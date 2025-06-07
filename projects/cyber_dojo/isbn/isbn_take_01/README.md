# ISBN Validator

A Rust library for validating ISBN-10 and ISBN-13 codes.

## ISBN - International Standard Book Number

There are two ISBN standards: ISBN-10 and ISBN-13.
This library supports both ISBN-13 and ISBN-10 validation.

Here are some valid examples of each:

### ISBN-10:
- 0471958697
- 0 471 60695 2
- 0-470-84525-2
- 0-321-14653-0

### ISBN-13:
- 9780470059029
- 978 0 471 48648 0
- 978-0596809485
- 978-0-13-149505-0
- 978-0-262-13472-9

ISBN-10 is made up of 9 digits plus a check digit (which may be 'X').
ISBN-13 is made up of 12 digits plus a check digit.
Spaces and hyphens may be included in a code, but are not significant.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
isbn = "0.1.0"
```

And then use the library in your code:

```rust
use isbn::validate_isbn;

fn main() {
    let isbn = "978-0-262-13472-9";
    if validate_isbn(isbn) {
        println!("{} is a valid ISBN", isbn);
    } else {
        println!("{} is not a valid ISBN", isbn);
    }
}
```

## Running the Application

```
cargo run
```

## Running Tests

```
cargo test
```
