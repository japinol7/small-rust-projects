use game_of_life::GameOfLife;

fn main() {
    let app_name = "Game of Life";
    println!("Start app {}", app_name);

    let grid_pattern = vec![
        "........".to_string(),
        "....*...".to_string(),
        "...**...".to_string(),
        ".....*..".to_string(),
    ];

    let mut game = GameOfLife::from_grid(4, 8, &grid_pattern);

    if let Err(err) = game.set_cell_representation("*", ".", "") {
        println!("Error: {}", err);
        println!("End app {}", app_name);
        return;
    }

    let generations_to_run = 4;
    for i in 0..generations_to_run {
        println!("\nGeneration {}", i + 1);
        print!("{}", game);
        game.calc_next_generation();
    }

    println!("\nEnd app {}", app_name);
}
