use count_coins::CountCoins;

#[test]
fn test_changes() {
    let tests = [(0, 0), (15, 6), (20, 9), (25, 13), (30, 18), (53, 49)];

    let cc = CountCoins::new();

    for (amount, expected) in tests {
        let result = cc.changes(amount);
        assert_eq!(
            result, expected,
            "Changes({}) = {}, want {}",
            amount, result, expected
        );
    }
}

#[test]
fn test_changes_100_cents_and_output() {
    let cc = CountCoins::new();
    let result = cc.changes(100);
    let expected = 242;

    assert_eq!(
        result, expected,
        "Changes(100) = {}, want {}",
        result, expected
    );

    // Comment the following lines to remove the output result
    println!("Output: ");
    println!("How many ways are there to make change for a dollar ");
    println!("using these common coins? (1 dollar = 100 cents) ");
    println!("Result: {}", result);
}
