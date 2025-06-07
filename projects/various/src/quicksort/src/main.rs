use quicksort::quicksort;

fn main() {
    let app_name = "Quicksort";
    println!("Start app {} \n", app_name);

    // Integer example
    let mut numbers = vec![64, 34, -25, 14, 12, 22, 11, 90];
    println!("Original integers: {:?}", numbers);
    quicksort(&mut numbers);
    println!("Sorted integers: {:?}", numbers);

    // Float example
    let mut floats = vec![2.21, 3.44, 1.41, -0.75, 1.71, 0.51, 1.83];
    println!("\nOriginal floats: {:?}", floats);
    quicksort(&mut floats);
    println!("Sorted floats: {:?}", floats);

    // String example
    let mut words = vec!["banana", "eye", "reload", "holster", "arrow", "blue"];
    println!("\nOriginal strings: {:?}", words);
    quicksort(&mut words);
    println!("Sorted strings: {:?}", words);

    println!("\nEnd app {}", app_name);
}
