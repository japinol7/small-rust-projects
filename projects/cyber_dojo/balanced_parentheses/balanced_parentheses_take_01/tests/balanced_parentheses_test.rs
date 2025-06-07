use balanced_parentheses::are_parentheses_balanced;

#[test]
fn test_are_parentheses_balanced() {
    let tests = [
        ("empty string", "", true),
        ("simple parentheses", "()", true),
        ("simple braces", "{}", true),
        ("nested braces and parentheses", "{()}", true),
        ("complex nested", "{[()]}", true),
        ("alternative nesting", "[({})]", true),
        ("multiple pairs", "{}([])", true),
        ("complex valid", "{()}[[{}]]", true),
        ("unbalanced close", "[]]", false),
        ("incorrectly nested", "{{)(}}", false),
        ("mismatched", "({)}", false),
    ];

    for (name, input, expected) in tests {
        let got = are_parentheses_balanced(input);
        assert_eq!(
            got, expected,
            "{}: are_parentheses_balanced({:?}) = {}, want {}",
            name, input, got, expected
        );
    }
}
