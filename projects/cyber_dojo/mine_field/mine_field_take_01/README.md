# Mine Field

## Description

Implements a solution for the mine field program.
A field of N x M squares is represented by N lines 
of exactly M characters each.

- The character '*' represents a mine.
- The character '.' represents no-mine.

## Example Input

A 3 x 4 mine-field of 12 squares, 2 of which are mines:

```
3 4
*...
..*.
....
```

## Task

Your task is to write a program to accept this input and produce 
as output a hint-field of identical dimensions where each 
square is a * for a mine or the number of adjacent mine-squares 
if the square does not contain a mine.

## Example Output

For the above input:

```
*211
12*1
0111
```

## Usage

Run the program:
```
cargo run
```

Run the tests:
```
cargo test
```
