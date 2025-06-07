use isbn::validate_isbn;

fn main() {
    let app_name = "Validate ISBN-13 and ISBN-10";
    println!("Start app {}", app_name);

    // Check valid isbn-13
    let isbn = "978-0-262-13472-9";
    let res = validate_isbn(isbn);
    println!("Should be true.  Is '{}' a valid isbn?: {}", isbn, res);

    // Check invalid isbn-13
    let isbn = "978-0-262-13472-1";
    let res = validate_isbn(isbn);
    println!("Should be false. Is '{}' a valid isbn?: {}", isbn, res);

    // Check valid isbn-10
    let isbn = "0-8044-2957-X";
    let res = validate_isbn(isbn);
    println!("Should be true.  Is '{}' a valid isbn?: {}", isbn, res);

    // Check invalid isbn-10
    let isbn = "0-4A0-84525-2";
    let res = validate_isbn(isbn);
    println!("Should be false. Is '{}' a valid isbn?: {}", isbn, res);

    println!("End app {}", app_name);
}
