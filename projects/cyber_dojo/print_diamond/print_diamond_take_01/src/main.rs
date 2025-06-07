use print_diamond::Diamond;

fn main() {
    let app_name = "Print Diamond";
    println!("Start app {}", app_name);

    // Examples of diamond patterns
    let letters = vec!["A", "B", "C", "D", "E"];

    for letter in letters {
        println!("\nDiamond for letter {}:", letter);
        let diamond = Diamond::new(letter);
        println!("{}", diamond.to_string());
    }

    println!("\nEnd app {}", app_name);
}
