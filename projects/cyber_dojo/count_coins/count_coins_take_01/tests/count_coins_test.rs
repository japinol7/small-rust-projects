use rstest::rstest;

use count_coins::CountCoins;

#[rstest]
#[case::val_0(0, 0)]
#[case::val_15(15, 6)]
#[case::val_20(20, 9)]
#[case::val_25(25, 13)]
#[case::val_30(30, 18)]
#[case::val_53(53, 49)]
fn test_changes(#[case] amount: i32, #[case] expected: i32) {
    let cc = CountCoins::new();
    let result = cc.changes(amount);
    assert_eq!(
        result, expected,
        "Changes({}) = {}, expected: {}",
        amount, result, expected
    );
}

#[test]
fn test_changes_100_cents_and_output() {
    let cc = CountCoins::new();
    let result = cc.changes(100);
    let expected = 242;

    assert_eq!(
        result, expected,
        "Changes(100) = {}, expected: {}",
        result, expected
    );

    // Comment the following lines to remove the output result
    println!("Output: ");
    println!("How many ways are there to make change for a dollar ");
    println!("using these common coins? (1 dollar = 100 cents) ");
    println!("Result: {}", result);
}
