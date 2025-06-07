use game_of_life::GameOfLife;

#[test]
fn test_init() {
    let game = GameOfLife::new(5, 5);
    for y in 0..5 {
        for x in 0..5 {
            assert_eq!(
                game.get_cell(x, y),
                0,
                "Expected cell ({},{}) to be 0, got {}",
                x,
                y,
                game.get_cell(x, y)
            );
        }
    }
}

#[test]
fn test_set_and_get_cell() {
    let mut game = GameOfLife::new(5, 5);
    game.set_cell(1, 2, 1);
    assert_eq!(
        game.get_cell(1, 2),
        1,
        "Expected cell (1,2) to be 1, got {}",
        game.get_cell(1, 2)
    );
}

#[test]
fn test_count_neighbors() {
    let mut game = GameOfLife::new(5, 5);
    game.set_cell(1, 1, 1);
    game.set_cell(2, 1, 1);
    game.set_cell(1, 2, 1);

    let neighbors1 = game.count_neighbors(1, 1);
    assert_eq!(
        neighbors1, 2,
        "Expected cell (1,1) to have 2 neighbors, got {}",
        neighbors1
    );

    let neighbors2 = game.count_neighbors(2, 2);
    assert_eq!(
        neighbors2, 3,
        "Expected cell (2,2) to have 3 neighbors, got {}",
        neighbors2
    );
}

#[test]
fn test_step_underpopulation() {
    let mut game = GameOfLife::new(5, 5);
    game.set_cell(1, 1, 1);

    game.calc_next_generation();
    let _ = game.set_cell_representation("*", ".", "");

    assert_eq!(
        game.get_cell(1, 1),
        0,
        "Expected cell (1,1) to die from underpopulation, got {}",
        game.get_cell(1, 1)
    );
}

#[test]
fn test_step_overpopulation() {
    let mut game = GameOfLife::new(5, 5);
    game.set_cell(1, 1, 1);
    game.set_cell(0, 0, 1);
    game.set_cell(0, 1, 1);
    game.set_cell(0, 2, 1);
    game.set_cell(1, 0, 1);

    game.calc_next_generation();
    let _ = game.set_cell_representation("*", ".", "");

    assert_eq!(
        game.get_cell(1, 1),
        0,
        "Expected cell (1,1) to die from overpopulation, got {}",
        game.get_cell(1, 1)
    );
}

#[test]
fn test_step_reproduction() {
    let input = vec![
        "**...".to_string(),
        "*....".to_string(),
        ".....".to_string(),
        ".....".to_string(),
        ".....".to_string(),
    ];
    let mut game = GameOfLife::from_grid(5, 5, &input);
    let _ = game.set_cell_representation("*", ".", "");

    game.calc_next_generation();
    assert_eq!(
        game.get_cell(1, 1),
        1,
        "Expected cell (1,1) to become alive from reproduction, got {}",
        game.get_cell(1, 1)
    );
}

#[test]
fn test_next_generation_from_grid1() {
    let input = vec![
        "........".to_string(),
        "....*...".to_string(),
        "...**...".to_string(),
        ".....*..".to_string(),
    ];
    let expected = ["........", "...**...", "...***..", "....*..."];

    let mut game = GameOfLife::from_grid(4, 8, &input);
    let _ = game.set_cell_representation("*", ".", "");
    game.calc_next_generation();

    for y in 0..4 {
        for x in 0..8 {
            let expected_state = if expected[y].chars().nth(x) == Some('*') {
                1
            } else {
                0
            };
            assert_eq!(
                game.get_cell(x, y),
                expected_state,
                "At position ({},{}): expected {}, got {}",
                x,
                y,
                expected_state,
                game.get_cell(x, y)
            );
        }
    }
}

#[test]
fn test_next_generation_from_grid2() {
    let input = vec![
        "........".to_string(),
        "...**...".to_string(),
        ".*****..".to_string(),
        "........".to_string(),
        "........".to_string(),
    ];
    let expected = ["........", ".....*..", "..*..*..", "..***...", "........"];

    let mut game = GameOfLife::from_grid(5, 8, &input);
    let _ = game.set_cell_representation("*", ".", "");
    game.calc_next_generation();

    for y in 0..4 {
        for x in 0..8 {
            let expected_state = if expected[y].chars().nth(x) == Some('*') {
                1
            } else {
                0
            };
            assert_eq!(
                game.get_cell(x, y),
                expected_state,
                "At position ({},{}): expected {}, got {}",
                x,
                y,
                expected_state,
                game.get_cell(x, y)
            );
        }
    }
}

#[test]
fn test_next_generation_from_grid_custom_cell_repr() {
    let input = vec![
        "--------".to_string(),
        "---@@---".to_string(),
        "-@@@@@--".to_string(),
        "--------".to_string(),
        "--------".to_string(),
    ];
    let expected = ["--------", "-----@--", "--@--@--", "--@@@---", "--------"];

    let mut game = GameOfLife::from_grid(5, 8, &input);
    let _ = game.set_cell_representation("@", "-", " ");
    game.calc_next_generation();

    for y in 0..4 {
        for x in 0..8 {
            let expected_state = if expected[y].chars().nth(x) == Some('@') {
                1
            } else {
                0
            };
            assert_eq!(
                game.get_cell(x, y),
                expected_state,
                "At position ({},{}): expected {}, got {}",
                x,
                y,
                expected_state,
                game.get_cell(x, y)
            );
        }
    }
}

#[test]
fn test_step_overpopulation_custom_cell_repr() {
    let mut game = GameOfLife::new(5, 5);
    game.set_cell(1, 1, 1);
    game.set_cell(0, 0, 1);
    game.set_cell(0, 1, 1);
    game.set_cell(0, 2, 1);
    game.set_cell(1, 0, 1);

    game.calc_next_generation();
    let _ = game.set_cell_representation("@", "-", " ");

    assert_eq!(
        game.get_cell(1, 1),
        0,
        "Expected cell (1,1) to die from overpopulation, got {}",
        game.get_cell(1, 1)
    );
}
