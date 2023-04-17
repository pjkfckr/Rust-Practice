

// Topic: Data management using tuples

// * Use a function that returns a tuple
fn coordinate() -> (i32, i32) {
    (1, 7)
}


fn main() {
    // * Destructure the return value into two variables
    let (x, y) = coordinate();

    // * Use an if..else if..else block to determine what to print
    if  y > 5 {
        println!(">5");
    } else if y < 5 {
        println!("<5");
    } else {
        println!("=5");
    }
}
