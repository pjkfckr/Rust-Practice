

// Topic: Working with an enum

// * Use an enum with colo names an variants
enum Color {
    Red,
    Black,
    Blue,
}

// * Use a function to print the color name
// * The function must use the enum as a parameter

fn print_color(my_color: Color) {
    // * Use a match expression to determine which color
    match my_color {
        Color::Red => println!("red"),
        Color::Black => println!("black"),
        Color::Blue => println!("blue"),
    }
}

fn main() {
    print_color(Color::Blue);
}