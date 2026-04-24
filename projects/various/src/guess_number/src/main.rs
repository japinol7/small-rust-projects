use rand::RngExt;
use std::cmp::Ordering;
use std::io;

const MAX_GUESSES: u32 = 10;

fn performance_grade(guesses_used: u32) -> &'static str {
    match guesses_used {
        1..=2 => "Excellent",
        3..=5 => "Good",
        6..=8 => "Fair",
        9..=10 => "Barely made it",
        _ => "Unknown",
    }
}

fn main() {
    println!("Guess the number!");
    println!("You have {MAX_GUESSES} valid guesses.");

    let mut rng = rand::rng();
    let num_to_guess = rng.random_range(1..=100);
    let mut guesses_used: u32 = 0;

    while guesses_used < MAX_GUESSES {
        println!(
            "Guess #{}: Please input your guess:",
            guesses_used + 1
        );

        let mut guess = String::new();
        if let Err(err) = io::stdin().read_line(&mut guess) {
            eprintln!("Failed to read line: {err}");
            continue;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid whole number between 1 and 100.");
                continue;
            }
        };

        if !(1..=100).contains(&guess) {
            println!("Please enter a number between 1 and 100.");
            continue;
        }

        guesses_used += 1;

        println!("You guessed: {guess}");

        match guess.cmp(&num_to_guess) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                let grade = performance_grade(guesses_used);

                println!("You win!");
                println!("Guesses needed: {guesses_used}");
                println!("Performance: {grade}");
                return;
            }
        }
    }

    println!(
        "Sorry, you've used all {MAX_GUESSES} available guesses. \
        The number was {num_to_guess}."
    );
    println!("Performance: Failed");
}
