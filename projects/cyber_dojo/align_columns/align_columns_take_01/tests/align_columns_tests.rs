use align_columns::{COL_SEP, ColumnAlignment, FILL_CH, align_columns};
use rstest::rstest;

// Original text with $ as a column separator
const INPUT_TEXT_ORIG: &str = "\
Given$a$text$file$of$many$lines,$where$fields$within$a$line$\n\
are$delineated$by$a$single$'dollar'$character,$write$a$program\n\
that$aligns$each$column$of$fields$by$ensuring$that$words$in$each$\n\
column$are$separated$by$at$least$one$space.";

// Initialize input_text by replacing $ with COL_SEP
fn get_input_text() -> String {
    INPUT_TEXT_ORIG.replace("$", COL_SEP)
}

#[test]
fn test_align_columns_left() {
    let expected_orig = r#"Given  a          text      file   of     many     lines,     where    fields within  a  line
are    delineated by        a      single 'dollar' character, write    a      program
that   aligns     each      column of     fields   by         ensuring that   words   in each
column are        separated by     at     least    one        space.  "#;

    let expected = expected_orig.replace(" ", FILL_CH);

    let result = align_columns(&get_input_text(), ColumnAlignment::Left);
    assert_eq!(result, expected);
}

#[test]
fn test_align_columns_right() {
    let expected_orig = r#" Given          a      text   file     of     many     lines,    where fields  within  a line
   are delineated        by      a single 'dollar' character,    write      a program
  that     aligns      each column     of   fields         by ensuring   that   words in each
column        are separated     by     at    least        one   space."#;

    let expected = expected_orig.replace(" ", FILL_CH);

    let result = align_columns(&get_input_text(), ColumnAlignment::Right);
    assert_eq!(result, expected);
}

#[test]
fn test_align_columns_center() {
    let expected_orig = r#"Given      a        text     file    of     many     lines,    where   fields within  a  line
 are   delineated    by       a    single 'dollar' character,  write     a    program
 that    aligns     each    column   of    fields      by     ensuring  that   words  in each
column    are     separated   by     at    least      one      space. "#;

    let expected = expected_orig.replace(" ", FILL_CH);

    let result = align_columns(&get_input_text(), ColumnAlignment::Center);
    assert_eq!(result, expected);
}

#[test]
fn test_align_columns_empty() {
    let result = align_columns("", ColumnAlignment::Left);
    let expected = "";
    assert_eq!(result, expected);
}

#[rstest]
#[case(ColumnAlignment::Left)]
#[case(ColumnAlignment::Right)]
#[case(ColumnAlignment::Center)]
fn test_alignment_with_different_types(#[case] alignment: ColumnAlignment) {
    let input = "a$bc$def\nghi$j$klmno";
    let result = align_columns(&input.replace("$", COL_SEP), alignment);
    assert!(
        !result.is_empty(),
        "Should return non-empty result for valid input"
    );
}
