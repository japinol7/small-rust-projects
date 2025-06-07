use std::fmt::Display;

const START_LETTER: char = 'A';
const SEPARATOR: &str = " ";

/// Diamond represents a letter diamond pattern
pub struct Diamond {
    letter: char,
    pattern: Vec<String>,
}

impl Diamond {
    /// Creates a new diamond pattern for the given letter
    pub fn new(letter: &str) -> Self {
        let mut diamond = Diamond {
            letter: '\0',
            pattern: Vec::new(),
        };

        // Check if the input is a single character
        let chars: Vec<char> = letter.chars().collect();
        if chars.len() != 1 {
            return diamond;
        }

        diamond.letter = chars[0].to_ascii_uppercase();

        if !diamond.is_input_valid() {
            return diamond;
        }

        if diamond.letter == START_LETTER {
            diamond.pattern = vec![START_LETTER.to_string()];
            return diamond;
        }

        diamond.create_diamond_pattern();
        diamond
    }

    /// Checks if the input letter is valid for creating a diamond
    fn is_input_valid(&self) -> bool {
        self.letter != '\0' && self.letter >= 'A' && self.letter <= 'Z'
    }

    /// Adds lines to the diamond pattern from A to the specified letter
    fn add_lines_until_middle_of_diamond(&mut self, width: usize, len_to_start_letter: usize) {
        // Add the first line with just 'A'
        self.pattern.push(center(&START_LETTER.to_string(), width));

        // Add lines from 'B' to the specified letter
        for i in 1..=len_to_start_letter {
            let ch = (START_LETTER as u8 + i as u8) as char;
            let spaces = 2 * i - 1;
            let line = format!("{}{}{}", ch, SEPARATOR.repeat(spaces), ch);
            self.pattern.push(center(&line, width));
        }
    }

    /// Creates the diamond pattern
    fn create_diamond_pattern(&mut self) {
        let len_to_start_letter = self.letter as usize - START_LETTER as usize;
        let width = 2 * len_to_start_letter + 1;

        self.add_lines_until_middle_of_diamond(width, len_to_start_letter);

        // Add mirrored lines to complete the diamond
        let upper_half = self.pattern.clone();
        for i in (0..upper_half.len() - 1).rev() {
            self.pattern.push(upper_half[i].clone());
        }
    }
}

impl Display for Diamond {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pattern.join("\n"))
    }
}

/// Centers a string within a given width
fn center(s: &str, width: usize) -> String {
    if s.len() >= width {
        return s.to_string();
    }

    let left_padding = (width - s.len()) / 2;
    let right_padding = width - s.len() - left_padding;

    format!(
        "{}{}{}",
        SEPARATOR.repeat(left_padding),
        s,
        SEPARATOR.repeat(right_padding)
    )
}
