use fizzbuzz_plus::{fizzbuzz_range, to_string};

fn main() {
    let app_name = "FizzBuzz Plus";
    println!("Start app {}", app_name);

    let res = fizzbuzz_range(15);
    println!("{}", to_string(&res));

    println!("End app {}", app_name);
}
