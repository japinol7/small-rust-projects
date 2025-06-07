use roman_numerals_full::{from_roman, to_roman};

fn main() {
    let app_name = "Roman Numerals Full";
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
