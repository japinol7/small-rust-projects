use quicksort_int::quicksort;

fn main() {
    let app_name = "Quicksort for regular integers";
    println!("Start app {} \n", app_name);

    let mut numbers = vec![64, 34, -25, 14, 12, 22, 11, 90];
    println!("Original integers: {:?}", numbers);
    let sorted_numbers = quicksort(&mut numbers);
    println!("Sorted integers: {:?}", sorted_numbers);

    println!("\nEnd app {}", app_name);
}
