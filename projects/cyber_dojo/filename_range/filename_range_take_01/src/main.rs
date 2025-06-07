use filename_range::filename_range;

fn main() {
    let app_name = "Filename Range";
    println!("Start app {}", app_name);

    let filename = "wibble/test/hiker_spec.rb";
    let name_range = filename_range(filename);
    let wanted_name = &filename[name_range[0]..name_range[1]];
    println!("{} -> {:?} -> {}", filename, name_range, wanted_name);

    println!("End app {}", app_name);
}
