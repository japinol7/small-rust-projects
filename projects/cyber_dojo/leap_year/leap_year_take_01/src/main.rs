use leap_year::is_leap_year;

fn main() {
    let app_name = "Leap Year";
    println!("Start app: {}", app_name);

    let example_years = vec![2001, 1996, 1900, 2000, 2023, 2024];

    for year in example_years {
        let result = is_leap_year(year);
        println!("Is {} a leap year: {}", year, result);
    }

    println!("End app: {}", app_name);
}
