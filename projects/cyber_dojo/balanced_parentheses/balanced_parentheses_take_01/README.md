# Balanced Parentheses

A Rust program to determine if the parentheses (), brackets [], and braces {} in a string are balanced.

## Examples

- `{{)(}}` is not balanced because `)` comes before `(`
- `({)}` is not balanced because `)` is not balanced between `{}` and similarly the `{` is not balanced between `()`
- `[({})]` is balanced
- `{}([])` is balanced
- `{()}[[{}]]` is balanced

## Usage

```rust
use balanced_parentheses::are_parentheses_balanced;

fn main() {
    let input = "{[()]}";
    let is_balanced = are_parentheses_balanced(input);
    println!("Is '{}' balanced? {}", input, is_balanced);
}
```

## To run the program

```bash
cargo run
```

## To run the tests

```bash
cargo test
```
