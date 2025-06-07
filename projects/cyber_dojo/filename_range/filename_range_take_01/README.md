# Filename Range

Implements the Filename Range algorithm, which determines which part of a filename should be selected for editing.

## Features

- Determines the part of a filename that should be selected for editing
- Excludes file extensions from selection
- Excludes special words like "test", "tests", "spec", or "step" from selection
- Excludes directory paths from selection

## Usage

```rust
use filename_range::filename_range;

fn main() {
    let filename = "test/hiker_spec.rb";
    let range = filename_range(filename);

    if !range.is_empty() {
        let selection = &filename[range[0]..range[1]];
        println!("Selected part: {}", selection);
    }
}
```

## Running the Tests

To run the program:

```
cargo run
```

To run the tests:

```
cargo test
```
