use count_coins::CountCoins;

fn main() {
    let app_name = "Count coins";
    println!("Start app {}", app_name);

    let cc = CountCoins::new();
    let result = cc.changes(100);

    println!("How many ways are there to make change for a dollar");
    println!("using these common coins? (1 dollar = 100 cents)");
    println!("Result: {}", result);

    println!("End app {}", app_name);
}
