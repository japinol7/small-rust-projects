use align_columns::{COL_SEP, ColumnAlignment, align_columns};

fn main() {
    let app_name = "Align Columns";
    println!("Start app {}", app_name);

    let input_text_orig = "\
Given$a$text$file$of$many$lines,$where$fields$within$a$line$\n\
are$delineated$by$a$single$'dollar'$character,$write$a$program\n\
that$aligns$each$column$of$fields$by$ensuring$that$words$in$each$\n\
column$are$separated$by$at$least$one$space.";

    // Replace $ with COL_SEP constant
    let input_text = input_text_orig.replace("$", COL_SEP);

    println!("\nLeft-aligned text:");
    println!("{}", align_columns(&input_text, ColumnAlignment::Left));

    println!("\nCenter-aligned text:");
    println!("{}", align_columns(&input_text, ColumnAlignment::Center));

    println!("\nRight-aligned text:");
    println!("{}", align_columns(&input_text, ColumnAlignment::Right));

    println!("\nEnd app {}", app_name);
}
