use leap_year::is_leap_year;

#[test]
fn test_is_leap_year() {
    let tests = vec![
        (2001, false),
        (1996, true),
        (1900, false),
        (2000, true),
        (2023, false),
        (2024, true),
        (0, false),
    ];

    for (year, expected) in tests {
        let result = is_leap_year(year);
        assert_eq!(
            result, expected,
            "is_leap_year({}) = {}; want {}",
            year, result, expected
        );
    }
}
