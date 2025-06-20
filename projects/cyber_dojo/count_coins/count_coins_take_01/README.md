# Count Coins

Implements a program that calculates the number of ways to make change 
for a given amount using common US coins.

## Description

There are four types of common coins in US currency:
- Quarters (25 cents)
- Dimes (10 cents)
- Nickels (5 cents)
- Pennies (1 cent)

For example, there are 6 ways to make change for 15 cents:
- A dime and a nickel
- A dime and 5 pennies
- 3 nickels
- 2 nickels and 5 pennies
- A nickel and 10 pennies
- 15 pennies

## Usage

To run the program:

```bash
cargo run
```

To run the tests:

```bash
cargo test
```

## Structure

- `src/lib.rs`: Contains the implementation of the `CountCoins` struct and its methods
- `src/main.rs`: Entry point for the executable that calculates change for a dollar
- `tests/count_coins_test.rs`: Tests for the `CountCoins` implementation
