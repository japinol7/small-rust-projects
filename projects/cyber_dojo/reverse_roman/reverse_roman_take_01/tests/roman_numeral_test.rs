use reverse_roman::from_roman;

#[test]
fn test_from_roman() {
    let test_cases = [
        ("I", 1, false),
        ("II", 2, false),
        ("III", 3, false),
        ("IV", 4, false),
        ("V", 5, false),
        ("IX", 9, false),
        ("X", 10, false),
        ("XL", 40, false),
        ("L", 50, false),
        ("LXXIII", 73, false),
        ("XC", 90, false),
        ("XCIII", 93, false),
        ("C", 100, false),
        ("CD", 400, false),
        ("D", 500, false),
        ("CM", 900, false),
        ("M", 1000, false),
        ("MCMLXXXIV", 1984, false),
        ("MMXXIII", 2023, false),
        ("MMMCMXCIX", 3999, false),
        ("", 0, true),          // Error case
        ("MMMM", 0, true),      // Error case (4000)
        ("ABC", 0, true),       // Error case (invalid chars)
        ("MMMCMXCIY", 0, true), // Error case (invalid char Y)
    ];

    for (roman, expected, has_error) in test_cases {
        let result = from_roman(roman);
        if has_error {
            assert!(
                result.is_err(),
                "from_roman({}) expected an error, got Ok({})",
                roman,
                result.unwrap_or_default()
            );
        } else {
            match result {
                Ok(num) => {
                    assert_eq!(
                        num, expected,
                        "from_roman({}) = {}, expected {}",
                        roman, num, expected
                    );
                }
                Err(e) => {
                    panic!("from_roman({}): unexpected error: {}", roman, e);
                }
            }
        }
    }
}
