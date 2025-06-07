/// Constants for opening parentheses characters
const OPEN_MARKS: [char; 3] = ['(', '{', '['];

/// Constants for closing parentheses characters
const CLOSE_MARKS: [char; 3] = [')', '}', ']'];

/// Checks if the input string has balanced parentheses
///
/// This function determines if all parentheses, braces, and brackets in the input string
/// are properly balanced and nested.
///
/// # Examples
///
/// ```
/// use balanced_parentheses::are_parentheses_balanced;
///
/// assert_eq!(are_parentheses_balanced("()"), true);
/// assert_eq!(are_parentheses_balanced("({})"), true);
/// assert_eq!(are_parentheses_balanced("({)}"), false);
/// ```
pub fn are_parentheses_balanced(s: &str) -> bool {
    // Create a map of opening to closing marks
    let marks_map: std::collections::HashMap<char, char> = OPEN_MARKS
        .iter()
        .zip(CLOSE_MARKS.iter())
        .map(|(&open, &close)| (open, close))
        .collect();
    let mut open_marks_stack = Vec::new();

    for ch in s.chars() {
        // Check if it's an opening mark
        if OPEN_MARKS.contains(&ch) {
            open_marks_stack.push(marks_map[&ch]);
            continue;
        }

        // Check if it's a closing mark
        if CLOSE_MARKS.contains(&ch) {
            // Check if stack is empty or the closing mark doesn't match
            if open_marks_stack.is_empty() || open_marks_stack.pop().unwrap() != ch {
                return false;
            }
        }
    }

    // Stack should be empty for balanced parentheses
    open_marks_stack.is_empty()
}
