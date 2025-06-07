# Game Of Life

Calculate the next generation of Conway's Game of Life.

## Description

Your task is to write a program to calculate the next generation
of Conway's game of life, given any starting position.

You start with a two-dimensional grid of cells,
where each cell is either alive or dead.
The grid is finite, and no life can exist off the edges.
When calculating the next generation of the grid, follow these four rules:

1. Any live cell with fewer than two live neighbours dies,
   as if caused by underpopulation.
2. Any live cell with more than three live neighbours dies,
   as if by overcrowding.
3. Any live cell with two or three live neighbours lives
   on to the next generation.
4. Any dead cell with exactly three live neighbours becomes a live cell.

See example/1 and example/2

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

- `src/lib.rs`: The main library implementing the Game of Life
- `src/main.rs`: The executable entry point
- `tests/game_of_life_test.rs`: Tests for the Game of Life implementation
