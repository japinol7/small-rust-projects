use rstest::rstest;

use leap_year::is_leap_year;

#[rstest]
#[case::non_leap_year(2001, false)]
#[case::typical_leap_year(1996, true)]
#[case::century_non_leap_year(1900, false)]
#[case::century_leap_year(2000, true)]
#[case::recent_non_leap_year(2023, false)]
#[case::upcoming_leap_year(2024, true)]
#[case::invalid_year_zero(0, false)]
fn test_is_leap_year(#[case] year: i32, #[case] expected: bool) {
    let result = is_leap_year(year);
    assert_eq!(
        result, expected,
        "is_leap_year({}) = {}; expected: {}",
        year, result, expected
    );
}
