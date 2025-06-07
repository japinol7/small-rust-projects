use roman_numerals::to_roman;

fn main() {
    let app_name = "Roman Numerals";
    println!("Start app: {}", app_name);

    // Examples of number to Roman numeral conversion
    let nums = [1, 4, 9, 14, 42, 73, 99, 2023];
    println!("\nConverting numbers to Roman numerals:");
    for num in nums {
        match to_roman(num) {
            Ok(roman) => println!("{} -> {}", num, roman),
            Err(e) => eprintln!("Error converting {}: {}", num, e),
        }
    }

    println!("\nEnd app: {}", app_name);
}
