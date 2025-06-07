use print_diamond::Diamond;

#[test]
fn test_print_diamond_empty() {
    let test_cases = vec![("", ""), (" ", ""), ("ñ", ""), ("FG", "")];

    for (letter, expected) in test_cases {
        let result = Diamond::new(letter).to_string();
        assert_eq!(result, expected, "Diamond({}) failed", letter);
    }
}

#[test]
fn test_print_diamond_a() {
    let result = Diamond::new("A").to_string();
    let expected = "A";
    assert_eq!(result, expected);
}

#[test]
fn test_print_diamond_b() {
    let result = Diamond::new("B").to_string();
    let expected = " A \n".to_string() + "B B\n" + " A ";
    assert_eq!(result, expected);
}

#[test]
fn test_print_diamond_c() {
    let result = Diamond::new("C").to_string();
    let expected = "  A  \n".to_string() + " B B \n" + "C   C\n" + " B B \n" + "  A  ";
    assert_eq!(result, expected);
}

#[test]
fn test_print_diamond_d() {
    let result = Diamond::new("D").to_string();
    let expected = "   A   \n".to_string()
        + "  B B  \n"
        + " C   C \n"
        + "D     D\n"
        + " C   C \n"
        + "  B B  \n"
        + "   A   ";
    assert_eq!(result, expected);
}

#[test]
fn test_print_diamond_f() {
    let result = Diamond::new("F").to_string();
    let expected = "     A     \n".to_string()
        + "    B B    \n"
        + "   C   C   \n"
        + "  D     D  \n"
        + " E       E \n"
        + "F         F\n"
        + " E       E \n"
        + "  D     D  \n"
        + "   C   C   \n"
        + "    B B    \n"
        + "     A     ";
    assert_eq!(result, expected);
}

#[test]
fn test_print_diamond_e_and_output() {
    let diamond = Diamond::new("E");
    let result = diamond.to_string();
    let expected = "    A    \n".to_string()
        + "   B B   \n"
        + "  C   C  \n"
        + " D     D \n"
        + "E       E\n"
        + " D     D \n"
        + "  C   C  \n"
        + "   B B   \n"
        + "    A    ";
    assert_eq!(result, expected);

    // Print output for visual inspection
    println!("\nOutput:\n{}\n", result);
}
