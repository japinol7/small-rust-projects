/// Represents the alignment type for text columns
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnAlignment {
    Left,
    Right,
    Center,
}

/// Constants for column separator and fill character
pub const COL_SEP: &str = "$";
pub const FILL_CH: &str = " ";

/// Aligns the columns in the given text according to the specified alignment
pub fn align_columns(text: &str, alignment: ColumnAlignment) -> String {
    if text.is_empty() {
        return String::new();
    }

    // Split the text into lines and then into parts by the column separator
    let lines: Vec<&str> = text.split('\n').collect();
    let mut parts: Vec<Vec<&str>> = Vec::with_capacity(lines.len());

    for line in &lines {
        // Remove trailing the column separator if present
        // and split by the column separator
        let trimmed = line.trim_end_matches(COL_SEP);
        parts.push(trimmed.split(COL_SEP).collect());
    }

    // Find the max number of columns
    let max_cols = parts.iter().map(|part| part.len()).max().unwrap_or(0);

    // Store the max width for each column
    let mut widths = vec![0; max_cols];

    // Calculate the max width for each column
    for col_idx in 0..max_cols {
        for part in &parts {
            if col_idx < part.len() {
                widths[col_idx] = widths[col_idx].max(part[col_idx].len());
            }
        }
    }

    // Format each line according to the alignment
    let mut result = String::new();

    for (line_idx, part) in parts.iter().enumerate() {
        let mut line_result = String::new();

        for (col_idx, word) in part.iter().enumerate() {
            let padding = widths[col_idx] - word.len();

            match alignment {
                ColumnAlignment::Left => {
                    line_result.push_str(word);
                    line_result.push_str(&FILL_CH.repeat(padding));
                }
                ColumnAlignment::Right => {
                    line_result.push_str(&FILL_CH.repeat(padding));
                    line_result.push_str(word);
                }
                ColumnAlignment::Center => {
                    let left_pad = padding / 2;
                    let right_pad = padding - left_pad;
                    line_result.push_str(&FILL_CH.repeat(left_pad));
                    line_result.push_str(word);
                    line_result.push_str(&FILL_CH.repeat(right_pad));
                }
            }

            // Add space between columns, but not after the last one
            if col_idx < part.len() - 1 {
                line_result.push_str(FILL_CH);
            }
        }

        // Add the formatted line to the result
        result.push_str(&line_result);

        // Add a newline after each line except the last one
        if line_idx < parts.len() - 1 {
            result.push('\n');
        }
    }

    result
}
