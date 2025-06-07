use reverse_roman::from_roman;

fn main() {
    let app_name = "Reverse Roman";
    println!("Start app: {}", app_name);

    // Examples of Roman numeral to number conversion
    let roman_nums = ["I", "IV", "IX", "XIV", "XLII", "LXXIII", "XCIX", "MMXXIII"];
    println!("\nConverting Roman numerals to numbers:");
    for roman in roman_nums {
        match from_roman(roman) {
            Ok(num) => println!("{} -> {}", roman, num),
            Err(e) => eprintln!("Error converting {}: {}", roman, e),
        }
    }

    println!("\nEnd app: {}", app_name);
}
