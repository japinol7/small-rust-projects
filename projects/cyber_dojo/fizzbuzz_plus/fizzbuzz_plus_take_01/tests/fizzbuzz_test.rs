use rstest::rstest;

use fizzbuzz_plus::{fizzbuzz, fizzbuzz_range, to_string};

#[rstest]
#[case::regular_number_1(1, "1")]
#[case::regular_number_2(2, "2")]
#[case::fizz_for_3(3, "Fizz")]
#[case::regular_number_4(4, "4")]
#[case::buzz_for_5(5, "Buzz")]
#[case::fizz_for_multiple_of_3(6, "Fizz")]
#[case::buzz_for_multiple_of_5(10, "Buzz")]
#[case::regular_number_13(13, "Fizz")]
#[case::fizzbuzz_for_multiple_of_3_and_5(15, "FizzBuzz")]
#[case::regular_number_52(52, "Buzz")]
fn test_fizzbuzz(#[case] input: i32, #[case] expected: &str) {
    let result = fizzbuzz(input);
    assert_eq!(
        result, expected,
        "fizzbuzz({}) = '{}', expected: '{}'",
        input, result, expected
    );
}

#[rstest]
#[case::single_number(1, "1")]
#[case::two_numbers(2, "1\n2")]
#[case::complete_sequence_to_15(
    15,
    "1\n2\nFizz\n4\nBuzz\nFizz\n7\n8\nFizz\nBuzz\n11\nFizz\nFizz\n14\nFizzBuzz"
)]
fn test_fizzbuzz_range(#[case] input: i32, #[case] expected: &str) {
    let result = to_string(&fizzbuzz_range(input));
    assert_eq!(
        result, expected,
        "to_string(fizzbuzz_range({})) = '{}', expected: '{}'",
        input, result, expected
    );
}

#[test]
fn test_fizzbuzz_range_until_one_hundred() {
    let result = to_string(&fizzbuzz_range(100));

    let expected = concat!(
        "1\n2\nFizz\n4\nBuzz\nFizz\n7\n8\nFizz\nBuzz\n11\nFizz\nFizz\n14\n",
        "FizzBuzz\n16\n17\nFizz\n19\nBuzz\nFizz\n22\nFizz\nFizz\nBuzz\n26\n",
        "Fizz\n28\n29\nFizzBuzz\nFizz\nFizz\nFizz\nFizz\nFizzBuzz\nFizz\n",
        "Fizz\nFizz\nFizz\nBuzz\n41\nFizz\nFizz\n44\nFizzBuzz\n46\n47\n",
        "Fizz\n49\nBuzz\nFizzBuzz\nBuzz\nFizzBuzz\nFizzBuzz\nBuzz\nBuzz\n",
        "FizzBuzz\nBuzz\nBuzz\nFizzBuzz\n61\n62\nFizz\n64\n",
        "Buzz\nFizz\n67\n68\nFizz\nBuzz\n71\nFizz\nFizz\n74\nFizzBuzz\n76\n",
        "77\nFizz\n79\nBuzz\nFizz\n82\nFizz\nFizz\nBuzz\n86\nFizz\n88\n89\n",
        "FizzBuzz\n91\n92\nFizz\n94\nBuzz\nFizz\n97\n98\nFizz\nBuzz",
    );

    assert_eq!(
        result, expected,
        "fizzbuzz_range(100) produced incorrect result"
    );

    // Uncomment to see the output
    // println!("{}", result);
}
