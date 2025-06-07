//! Implementation of Conway's Game of Life

use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GameError {
    #[error("cell representation can only be set in initial generation")]
    NotInitialGeneration,
    #[error("alive cell representation must be a single character")]
    InvalidAliveCell,
    #[error("empty cell representation must be a single character")]
    InvalidEmptyCell,
}

type Result<T> = std::result::Result<T, GameError>;

/// Default representation for a living cell
pub const CELL_ALIVE_DEFAULT: &str = "*";
/// Default representation for an empty cell
pub const CELL_EMPTY_DEFAULT: &str = ".";
/// Default separator between cells
pub const SEPARATOR_DEFAULT: &str = "";

/// GameOfLife represents the game state
pub struct GameOfLife {
    rows: usize,
    cols: usize,
    grid: Vec<Vec<u8>>,
    initial_pattern: Vec<String>,
    is_initial_generation: bool,
    cell_alive: String,
    cell_empty: String,
    separator: String,
}

impl GameOfLife {
    /// Creates a new game with an empty grid of given dimensions
    pub fn new(rows: usize, cols: usize) -> Self {
        let grid = vec![vec![0; cols]; rows];
        let empty_pattern = vec![CELL_EMPTY_DEFAULT.repeat(cols); rows];

        Self {
            rows,
            cols,
            grid,
            initial_pattern: empty_pattern,
            is_initial_generation: true,
            cell_alive: CELL_ALIVE_DEFAULT.to_string(),
            cell_empty: CELL_EMPTY_DEFAULT.to_string(),
            separator: SEPARATOR_DEFAULT.to_string(),
        }
    }

    /// Creates a new game from a string grid representation
    pub fn from_grid(height: usize, width: usize, grid_str: &[String]) -> Self {
        let mut grid = vec![vec![0; width]; height];
        let initial_pattern = grid_str.to_vec();

        // Create grid using the default cell representation
        for (i, row) in grid_str.iter().enumerate().take(height) {
            for (j, ch) in row.chars().enumerate().take(width) {
                if ch.to_string() == CELL_ALIVE_DEFAULT {
                    grid[i][j] = 1;
                }
            }
        }

        Self {
            rows: height,
            cols: width,
            grid,
            initial_pattern,
            is_initial_generation: true,
            cell_alive: CELL_ALIVE_DEFAULT.to_string(),
            cell_empty: CELL_EMPTY_DEFAULT.to_string(),
            separator: SEPARATOR_DEFAULT.to_string(),
        }
    }

    /// Sets custom cell representation
    pub fn set_cell_representation(
        &mut self,
        alive: &str,
        empty: &str,
        separator: &str,
    ) -> Result<()> {
        if !self.is_initial_generation {
            return Err(GameError::NotInitialGeneration);
        }

        if alive.chars().count() > 1 {
            return Err(GameError::InvalidAliveCell);
        }

        if empty.chars().count() > 1 {
            return Err(GameError::InvalidEmptyCell);
        }

        self.cell_alive = alive.to_string();
        self.cell_empty = empty.to_string();
        self.separator = separator.to_string();

        // Recreate grid using custom cell representation
        for i in 0..self.rows {
            for j in 0..self.cols {
                if i < self.initial_pattern.len() && j < self.initial_pattern[i].len() {
                    if let Some(ch) = self.initial_pattern[i].chars().nth(j) {
                        if ch.to_string() == self.cell_alive {
                            self.grid[i][j] = 1;
                        } else {
                            self.grid[i][j] = 0;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Sets the state of a cell at the given coordinates
    pub fn set_cell(&mut self, x: usize, y: usize, state: u8) {
        if y < self.rows && x < self.cols {
            self.grid[y][x] = state;
        }
    }

    /// Returns the state of a cell at the given coordinates
    pub fn get_cell(&self, x: usize, y: usize) -> u8 {
        if y < self.rows && x < self.cols {
            self.grid[y][x]
        } else {
            0
        }
    }

    /// Counts the number of live neighbors for a cell
    pub fn count_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;

        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }

                let nx = (x as isize + i) as usize;
                let ny = (y as isize + j) as usize;

                if nx < self.cols && ny < self.rows {
                    count += self.grid[ny][nx];
                }
            }
        }

        count
    }

    /// Advances the game by one generation
    pub fn calc_next_generation(&mut self) {
        let mut new_grid = vec![vec![0; self.cols]; self.rows];

        // Enforce the game rules to determine life and death
        for y in 0..self.rows {
            for x in 0..self.cols {
                let neighbors = self.count_neighbors(x, y);

                if self.grid[y][x] == 1 {
                    // Cell is alive
                    if neighbors < 2 || neighbors > 3 {
                        // Cell dies by isolation or by overpopulation
                        new_grid[y][x] = 0;
                    } else {
                        // Cell survives
                        new_grid[y][x] = 1;
                    }
                } else {
                    // Cell is dead
                    if neighbors == 3 {
                        // Cell becomes alive
                        new_grid[y][x] = 1;
                    } else {
                        // Cell stays dead
                        new_grid[y][x] = 0;
                    }
                }
            }
        }

        self.grid = new_grid;
        self.is_initial_generation = false;
    }
}

impl fmt::Display for GameOfLife {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.grid {
            for cell in row {
                if *cell == 1 {
                    write!(f, "{}{}", self.cell_alive, self.separator)?
                } else {
                    write!(f, "{}{}", self.cell_empty, self.separator)?
                }
            }
            writeln!(f)?
        }
        Ok(())
    }
}
