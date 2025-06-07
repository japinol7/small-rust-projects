use roman_numerals_full::{from_roman, to_roman};

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

// Test round-trip conversion
#[test]
fn test_round_trip() {
    // Test every 100 numbers to keep test runtime reasonable
    for i in (1..=3999).step_by(100) {
        let roman = match to_roman(i) {
            Ok(r) => r,
            Err(e) => {
                panic!("to_roman({}) unexpected error: {}", i, e);
            }
        };

        let num = match from_roman(&roman) {
            Ok(n) => n,
            Err(e) => {
                panic!("from_roman({}) unexpected error: {}", roman, e);
            }
        };

        assert_eq!(num, i, "Round trip failed: {} -> {} -> {}", i, roman, num);
    }
}
