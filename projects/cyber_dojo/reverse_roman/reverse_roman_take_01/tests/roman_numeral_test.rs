use rstest::rstest;

use reverse_roman::from_roman;

#[rstest]
#[case::single_numeral_i("I", 1)]
#[case::two_numerals_ii("II", 2)]
#[case::three_numerals_iii("III", 3)]
#[case::subtractive_iv("IV", 4)]
#[case::single_numeral_v("V", 5)]
#[case::subtractive_ix("IX", 9)]
#[case::single_numeral_x("X", 10)]
#[case::subtractive_xl("XL", 40)]
#[case::single_numeral_l("L", 50)]
#[case::compound_lxxiii("LXXIII", 73)]
#[case::subtractive_xc("XC", 90)]
#[case::compound_xciii("XCIII", 93)]
#[case::single_numeral_c("C", 100)]
#[case::subtractive_cd("CD", 400)]
#[case::single_numeral_d("D", 500)]
#[case::subtractive_cm("CM", 900)]
#[case::single_numeral_m("M", 1000)]
#[case::year_1984("MCMLXXXIV", 1984)]
#[case::year_2023("MMXXIII", 2023)]
#[case::max_value_3999("MMMCMXCIX", 3999)]
fn test_valid_roman_numerals(#[case] input: &str, #[case] expected: i32) {
    let result = from_roman(input);
    assert!(result.is_ok(), "Expected Ok for {}", input);
    let result = result.unwrap();
    assert_eq!(
        result, expected,
        "from_roman({}) = {}, expected {}",
        input, result, expected
    );
}

#[rstest]
#[case::empty_string("")]
#[case::exceeds_max_value("MMMM")]
#[case::invalid_characters("ABC")]
#[case::invalid_char_in_valid_numeral("MMMCMXCIY")]
fn test_invalid_roman_numerals(#[case] input: &str) {
    let result = from_roman(input);
    assert!(
        result.is_err(),
        "from_roman({}) expected an error, got Ok({})",
        input,
        result.unwrap_or_default()
    );
}
