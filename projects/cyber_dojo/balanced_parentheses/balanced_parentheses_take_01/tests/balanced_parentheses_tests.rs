use rstest::rstest;

use balanced_parentheses::are_parentheses_balanced;

#[rstest]
#[case::empty_string("", true, "empty string")]
#[case::single_pair_parens("()", true, "single pair of parentheses")]
#[case::single_pair_braces("{}", true, "single pair of braces")]
#[case::braces_containing_parens("{()}", true, "braces containing parentheses")]
#[case::nested_mixed_brackets("{[()]}", true, "nested mixed brackets")]
#[case::alter_bracket_types("[({})]", true, "alternating bracket types")]
#[case::seq_bracket_pairs("{}([])", true, "sequential bracket pairs")]
#[case::complex_nested_structure("{()}[[{}]]", true, "complex nested structure")]
#[case::extra_closing_bracket("[]]", false, "extra closing bracket")]
#[case::incorrectly_nested_brackets("{{)(}}", false, "incorrectly nested brackets")]
#[case::mismatched_bracket_pairs("({)}", false, "mismatched bracket pairs")]
fn test_are_parentheses_balanced(
    #[case] input: &str,
    #[case] expected: bool,
    #[case] test_name: &str,
) {
    let got = are_parentheses_balanced(input);
    assert_eq!(
        got, expected,
        "{}: are_parentheses_balanced({:?}) = {}, expected: {}",
        test_name, input, got, expected
    );
}
