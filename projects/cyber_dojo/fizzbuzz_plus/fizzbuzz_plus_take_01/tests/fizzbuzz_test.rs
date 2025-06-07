use fizzbuzz_plus::{fizzbuzz, fizzbuzz_range, to_string};

#[test]
fn test_fizzbuzz() {
    let tests = [
        (1, "1"),
        (2, "2"),
        (3, "Fizz"),
        (4, "4"),
        (5, "Buzz"),
        (6, "Fizz"),
        (10, "Buzz"),
        (13, "Fizz"),
        (15, "FizzBuzz"),
        (52, "Buzz"),
    ];

    for (input, expected) in tests {
        let result = fizzbuzz(input);
        assert_eq!(
            result, expected,
            "fizzbuzz({}) = {:?}, want {:?}",
            input, result, expected
        );
    }
}

#[test]
fn test_fizzbuzz_range() {
    let tests = [
        (1, "1"),
        (2, "1\n2"),
        (
            15,
            "1\n2\nFizz\n4\nBuzz\nFizz\n7\n8\nFizz\nBuzz\n11\n\
            Fizz\nFizz\n14\nFizzBuzz",
        ),
    ];

    for (input, expected) in tests {
        let result = to_string(&fizzbuzz_range(input));
        assert_eq!(
            result, expected,
            "to_string(fizzbuzz_range({})) = {:?}, want {:?}",
            input, result, expected
        );
    }
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
