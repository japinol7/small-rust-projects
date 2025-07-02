use rstest::rstest;

use roman_numerals::to_roman;

#[rstest]
#[case::one(1, "I")]
#[case::two(2, "II")]
#[case::three(3, "III")]
#[case::four(4, "IV")]
#[case::five(5, "V")]
#[case::nine(9, "IX")]
#[case::ten(10, "X")]
#[case::forty(40, "XL")]
#[case::fifty(50, "L")]
#[case::seventy_three(73, "LXXIII")]
#[case::ninety(90, "XC")]
#[case::ninety_three(93, "XCIII")]
#[case::hundred(100, "C")]
#[case::four_hundred(400, "CD")]
#[case::five_hundred(500, "D")]
#[case::nine_hundred(900, "CM")]
#[case::thousand(1000, "M")]
#[case::year_1984(1984, "MCMLXXXIV")]
#[case::year_2023(2023, "MMXXIII")]
#[case::max_value_3999(3999, "MMMCMXCIX")]
fn test_valid_roman_numerals(#[case] input: i32, #[case] expected: &str) {
    let result = to_roman(input).unwrap();
    assert_eq!(
        result, expected,
        "to_roman({}) = {}, expected: {}",
        input, result, expected
    );
}

#[rstest]
#[case::zero(0)]
#[case::negative(-1)]
#[case::too_large(4000)]
fn test_invalid_roman_numerals(#[case] input: i32) {
    let result = to_roman(input);
    assert!(
        result.is_err(),
        "to_roman({}) expected an error, got Ok({})",
        input,
        result.unwrap_or_default()
    );
}
