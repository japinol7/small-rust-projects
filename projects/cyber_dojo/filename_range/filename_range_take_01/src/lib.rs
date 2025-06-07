use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref CHARS_TO_EXCLUDE_LEFT: HashMap<char, bool> = {
        let mut map = HashMap::new();
        map.insert('\\', true);
        map.insert('/', true);
        map
    };
    static ref WORDS_TO_KEEP: Vec<&'static str> = vec!["tests", "test", "spec", "step",];
    static ref SEPARATORS: HashMap<char, bool> = {
        let mut map = HashMap::new();
        map.insert('-', true);
        map.insert('_', true);
        map.insert('.', true);
        map
    };
}

fn is_separator(ch: char) -> bool {
    SEPARATORS.get(&ch).copied().unwrap_or(false)
}

fn keep_word_on_the_right(
    text: &str,
    word_to_keep: &str,
    left_mark: usize,
    right_mark: usize,
) -> usize {
    if let Some(word_idx) = text.find(word_to_keep) {
        if 0 < word_idx && word_idx < right_mark {
            let mut new_right_mark = word_idx;
            if new_right_mark > 0 {
                let prev_char = text.chars().nth(new_right_mark - 1).unwrap_or('\0');
                if is_separator(prev_char) {
                    new_right_mark -= 1;
                }
            }
            return new_right_mark + left_mark;
        }
    }
    right_mark
}

fn keep_word_on_the_left(text: &str, word_to_keep: &str, left_mark: usize) -> usize {
    if let Some(word_idx) = text.find(word_to_keep) {
        if word_idx >= left_mark {
            let mut new_left_mark = word_idx + word_to_keep.len();
            if new_left_mark < text.len() {
                let next_char = text.chars().nth(new_left_mark).unwrap_or('\0');
                if is_separator(next_char) {
                    new_left_mark += 1;
                }
            }
            return new_left_mark;
        }
    }
    left_mark
}

/// Returns the range of the filename that should be selected for editing.
///
/// Given a filename, this function determines which part of the filename
/// should be selected for editing based on certain rules:
/// 1. The extension is excluded from selection
/// 2. If the filename contains words like "test", "tests", "spec", or "step",
///    these words and their separators are excluded
/// 3. Directory paths are excluded
///
/// # Arguments
/// * `filename` - The filename to analyze
///
/// # Returns
/// A vector containing two integers: the start and end indices of the selection.
/// For an empty filename, an empty vector is returned.
pub fn filename_range(filename: &str) -> Vec<usize> {
    if filename.is_empty() {
        return Vec::new();
    }

    let name = filename.to_lowercase();

    // Remove chars from the left to the last char to exclude
    let mut left_mark = 0;
    for (i, ch) in name.chars().rev().enumerate() {
        if CHARS_TO_EXCLUDE_LEFT.get(&ch).copied().unwrap_or(false) {
            left_mark = name.len() - i;
            break;
        }
    }

    // Remove chars from the right of the first dot char
    let mut right_mark = name.len();
    for (i, ch) in name.chars().enumerate() {
        if ch == '.' {
            right_mark = i;
            break;
        }
    }

    // Remove words to keep on the right and their separators
    let name_tp = &name[left_mark..];
    for word in WORDS_TO_KEEP.iter() {
        right_mark = keep_word_on_the_right(name_tp, word, left_mark, right_mark);
    }

    // Remove words to keep on the left and their separators
    let name_tp = &name[..right_mark];
    for word in WORDS_TO_KEEP.iter() {
        left_mark = keep_word_on_the_left(name_tp, word, left_mark);
    }

    vec![left_mark, right_mark]
}
