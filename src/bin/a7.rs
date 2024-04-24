// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

// * Use an enum with color names as variants
enum Color {
    Red,
    Green,
    Blue,
    Black,
}

// * Use a function to print the color name
// * The function must use the enum as a parameter
fn print_color(color: Color) {
    // * Use a match expression to determine which color
    match color {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
        Color::Black => println!("Black"),
    }
}

fn main() {
    print_color(Color::Black);
}
