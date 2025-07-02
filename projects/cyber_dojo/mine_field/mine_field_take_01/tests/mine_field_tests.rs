use mine_field::MineField;

#[test]
fn test_resolve_1() {
    let board = "3 4\n\
        *...\n\
        ..*.\n\
        ....";

    let expected = "\
        *211\n\
        12*1\n\
        0111";

    let mf = MineField::new(board).unwrap();
    let result = mf.resolve();

    assert_eq!(result, expected);
}

#[test]
fn test_resolve_2() {
    let board = "5 4\n\
        *...\n\
        ..*. \n\
        ...*\n\
        ....\n\
        .*.. ";

    let expected = "\
        *211\n\
        12*2\n\
        012*\n\
        1121\n\
        1*10";

    let mf = MineField::new(board).unwrap();
    let result = mf.resolve();

    assert_eq!(result, expected);
}
