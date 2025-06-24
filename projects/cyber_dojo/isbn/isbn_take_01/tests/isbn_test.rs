use rstest::rstest;

use isbn::validate_isbn;

#[rstest]
#[case::basic_isbn13("9780470059029", true)]
#[case::isbn13_with_spaces("978 0 471 48648 0", true)]
#[case::isbn13_with_hyphens("978-0596809485", true)]
#[case::isbn13_complex_hyphens("978-0-13-149505-0", true)]
#[case::isbn13_valid_checksum("978-0-262-13472-9", true)]
#[case::isbn13_invalid_checksum1("978-0-262-13472-1", false)]
#[case::isbn13_invalid_checksum2("978-0-262-13472-2", false)]
#[case::isbn13_with_letter("978 0 A 471 48648 0", false)]
#[case::isbn13_with_symbol("978 0 * 471 48648 0", false)]
#[case::isbn13_with_x("978-0-262-13472-X", false)]
#[case::isbn13_too_short("978-13472-2", false)]
#[case::isbn13_with_invalid_char("978-0-A62-13472-1", false)]
fn test_validate_isbn13(#[case] input_isbn: &str, #[case] expected: bool) {
    let result = validate_isbn(input_isbn);
    assert_eq!(result, expected, "Failed for ISBN-13: {}", input_isbn);
}

#[rstest]
#[case::basic_isbn10("0471958697", true)]
#[case::isbn10_with_spaces("0 471 60695 2", true)]
#[case::isbn10_with_hyphens("0-470-84525-2", true)]
#[case::isbn10_complex_format("0-321-14653-0", true)]
#[case::isbn10_with_valid_x("0-8044-2957-X", true)]
#[case::isbn10_with_x_at_end("0-9752298-0-X", true)]
#[case::isbn10_invalid_char("0-8044-2957-D", false)]
#[case::isbn10_invalid_checksum("0-470-84525-3", false)]
#[case::isbn10_with_letter("0-4A0-84525-2", false)]
#[case::isbn10_with_symbol("0-470-*4525-2", false)]
#[case::isbn10_too_short("0-470-8425-2", false)]
fn test_validate_isbn10(#[case] input_isbn: &str, #[case] expected: bool) {
    let result = validate_isbn(input_isbn);
    assert_eq!(result, expected, "Failed for ISBN-10: {}", input_isbn);
}
