mod lcd_digit_cell;
mod lcd_digits;

use lcd_digits::LcdDigits;

fn main() {
    let app_name = "LCD Digits";
    println!("Start app {}", app_name);

    let input_number = 1234567890;
    let lcd = LcdDigits::new();
    match lcd.generate(input_number, None) {
        Ok(result) => {
            println!("LCD Digits for {}\n{}", input_number, result);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    println!("End app {}", app_name);
}
