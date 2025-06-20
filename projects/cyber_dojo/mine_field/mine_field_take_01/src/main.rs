use mine_field::MineField;

fn main() {
    let app_name = "Mine field";
    println!("Start app {}", app_name);

    // Example usage
    let board = "3 4\n\
        *...\n\
        ..*.\n\
        ....";

    match MineField::new(board) {
        Ok(mf) => {
            let result = mf.resolve();

            println!("Input:");
            println!("{}", board);
            println!("\nOutput:");
            println!("{}", result);
        }
        Err(e) => {
            eprintln!("Error creating minefield: {}", e);
        }
    }

    println!("End app {}", app_name);
}
