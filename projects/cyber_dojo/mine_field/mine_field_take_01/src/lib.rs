use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MineFieldError {
    #[error("Failed to parse dimensions: {0}")]
    ParseError(String),
}

/// Cell represents a cell in the minefield
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Mine,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '*' => Cell::Mine,
            _ => Cell::Empty,
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Mine => write!(f, "*"),
        }
    }
}

/// MineField represents a field with mines
#[derive(Debug)]
pub struct MineField {
    n_rows: usize,
    n_columns: usize,
    board: Vec<Vec<Cell>>,
}

impl MineField {
    /// Creates a new minefield from the given board string
    pub fn new(board: &str) -> Result<Self, MineFieldError> {
        let lines: Vec<&str> = board.lines().collect();

        // Parse dimensions from the first line
        let dimensions: Vec<&str> = lines[0].split_whitespace().collect();
        let n_rows = dimensions[0]
            .parse::<usize>()
            .map_err(|e| MineFieldError::ParseError(e.to_string()))?;
        let n_columns = dimensions[1]
            .parse::<usize>()
            .map_err(|e| MineFieldError::ParseError(e.to_string()))?;

        // Parse the board
        let mut board_grid = vec![vec![Cell::Empty; n_columns]; n_rows];
        for i in 0..n_rows {
            let line = lines[i + 1].trim();
            for (j, c) in line.chars().take(n_columns).enumerate() {
                board_grid[i][j] = Cell::from(c);
            }
        }

        Ok(MineField {
            n_rows,
            n_columns,
            board: board_grid,
        })
    }

    /// Counts the number of mine neighbors for a given cell
    fn count_neighbours(&self, n_row: usize, n_column: usize) -> u8 {
        let mut count = 0;

        let row_start = n_row.saturating_sub(1);
        let row_end = (n_row + 2).min(self.n_rows);
        let col_start = n_column.saturating_sub(1);
        let col_end = (n_column + 2).min(self.n_columns);

        for y in row_start..row_end {
            for x in col_start..col_end {
                // Skip the cell itself and count only if it's a mine
                if (x != n_column || y != n_row) && self.board[y][x] == Cell::Mine {
                    count += 1;
                }
            }
        }

        count
    }

    /// Calculates the hint field based on the minefield
    pub fn resolve(&self) -> String {
        let mut result = String::new();

        for n_row in 0..self.n_rows {
            for n_column in 0..self.n_columns {
                match self.board[n_row][n_column] {
                    Cell::Mine => result.push('*'),
                    Cell::Empty => {
                        let count = self.count_neighbours(n_row, n_column);
                        result.push_str(&count.to_string());
                    }
                }
            }

            // Add a newline if not the last row
            if n_row < self.n_rows - 1 {
                result.push('\n');
            }
        }

        result
    }
}

impl fmt::Display for MineField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}\n", self.n_rows, self.n_columns)?;

        for row in &self.board {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
