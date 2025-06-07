use roman_numerals::to_roman;

#[test]
fn test_to_roman() {
    let test_cases = [
        (1, "I", false),
        (2, "II", false),
        (3, "III", false),
        (4, "IV", false),
        (5, "V", false),
        (9, "IX", false),
        (10, "X", false),
        (40, "XL", false),
        (50, "L", false),
        (73, "LXXIII", false),
        (90, "XC", false),
        (93, "XCIII", false),
        (100, "C", false),
        (400, "CD", false),
        (500, "D", false),
        (900, "CM", false),
        (1000, "M", false),
        (1984, "MCMLXXXIV", false),
        (2023, "MMXXIII", false),
        (3999, "MMMCMXCIX", false),
        (0, "", true),    // Error case
        (4000, "", true), // Error case
        (-1, "", true),   // Error case
    ];

    for (num, expected, has_error) in test_cases {
        let result = to_roman(num);
        if has_error {
            assert!(
                result.is_err(),
                "to_roman({}) expected an error, got Ok({})",
                num,
                result.unwrap_or_default()
            );
        } else {
            match result {
                Ok(roman) => {
                    assert_eq!(
                        roman, expected,
                        "to_roman({}) = {}, expected {}",
                        num, roman, expected
                    );
                }
                Err(e) => {
                    panic!("to_roman({}): unexpected error: {}", num, e);
                }
            }
        }
    }
}
