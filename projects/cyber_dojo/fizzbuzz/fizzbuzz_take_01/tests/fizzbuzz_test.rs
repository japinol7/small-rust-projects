use fizzbuzz::{fizzbuzz, fizzbuzz_range, to_string};

#[test]
fn test_fizzbuzz() {
    let tests = [
        ("num 1", 1, "1"),
        ("num 2", 2, "2"),
        ("num 3", 3, "Fizz"),
        ("num 4", 4, "4"),
        ("num 5", 5, "Buzz"),
        ("num 6", 6, "Fizz"),
        ("num 10", 10, "Buzz"),
        ("num 13", 13, "13"),
        ("num 15", 15, "FizzBuzz"),
        ("num 52", 52, "52"),
    ];

    for (name, input, expected) in tests {
        let result = fizzbuzz(input);
        assert_eq!(
            result, expected,
            "{}: fizzbuzz({}) = '{}', want '{}'",
            name, input, result, expected
        );
    }
}

#[test]
fn test_fizzbuzz_range() {
    let tests = [
        ("range 1", 1, "1"),
        ("range 2", 2, "1\n2"),
        (
            "range 15",
            15,
            "1\n2\nFizz\n4\nBuzz\nFizz\n7\n8\nFizz\nBuzz\n11\n\
            Fizz\n13\n14\nFizzBuzz",
        ),
    ];

    for (name, input, expected) in tests {
        let result = to_string(&fizzbuzz_range(input));
        assert_eq!(
            result, expected,
            "{}: to_string(fizzbuzz_range({})) = '{}', want '{}'",
            name, input, result, expected
        );
    }
}

#[test]
fn test_fizzbuzz_range_until_one_hundred() {
    let result = to_string(&fizzbuzz_range(100));

    let expected = concat!(
        "1\n2\nFizz\n4\nBuzz\nFizz\n7\n8\nFizz\nBuzz\n11\nFizz\n13\n14\n",
        "FizzBuzz\n16\n17\nFizz\n19\nBuzz\nFizz\n22\n23\nFizz\nBuzz\n26\n",
        "Fizz\n28\n29\nFizzBuzz\n31\n32\nFizz\n34\nBuzz\nFizz\n37\n38\nFizz\n",
        "Buzz\n41\nFizz\n43\n44\nFizzBuzz\n46\n47\nFizz\n49\nBuzz\nFizz\n",
        "52\n53\nFizz\nBuzz\n56\nFizz\n58\n59\nFizzBuzz\n61\n62\nFizz\n64\n",
        "Buzz\nFizz\n67\n68\nFizz\nBuzz\n71\nFizz\n73\n74\nFizzBuzz\n76\n",
        "77\nFizz\n79\nBuzz\nFizz\n82\n83\nFizz\nBuzz\n86\nFizz\n88\n89\n",
        "FizzBuzz\n91\n92\nFizz\n94\nBuzz\nFizz\n97\n98\nFizz\nBuzz"
    );

    assert_eq!(
        result, expected,
        "fizzbuzz_range(100) produced incorrect result"
    );

    // Uncomment to see the output
    // println!("{}", result);
}
