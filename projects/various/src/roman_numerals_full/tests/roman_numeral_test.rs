use rstest::rstest;

use roman_numerals_full::{from_roman, to_roman};

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
fn test_valid_to_roman_numerals(#[case] input: i32, #[case] expected: &str) {
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
fn test_invalid_to_roman_numerals(#[case] input: i32) {
    let result = to_roman(input);
    assert!(
        result.is_err(),
        "to_roman({}) expected an error, got Ok({})",
        input,
        result.unwrap_or_default()
    );
}

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
fn test_valid_from_roman_numerals(#[case] input: &str, #[case] expected: i32) {
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
fn test_invalid_from_roman_numerals(#[case] input: &str) {
    let result = from_roman(input);
    assert!(
        result.is_err(),
        "from_roman({}) expected an error, got Ok({})",
        input,
        result.unwrap_or_default()
    );
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
