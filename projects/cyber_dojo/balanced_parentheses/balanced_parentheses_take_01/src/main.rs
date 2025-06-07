use balanced_parentheses::are_parentheses_balanced;

fn main() {
    let app_name = "Balanced Parentheses";
    println!("Start app {}", app_name);

    let input_str = "((()))";
    let res = are_parentheses_balanced(input_str);
    println!("Are parentheses balanced for : '{}' ? {}", input_str, res);

    let input_str = "(()))";
    let res = are_parentheses_balanced(input_str);
    println!("Are parentheses balanced for : '{}' ? {}", input_str, res);

    println!("End app {}", app_name);
}
