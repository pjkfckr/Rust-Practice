
// Topic: Working with expressions

// Notes:
// * Use a boolean variable set to an expression
//   that determines whether the value is >100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message to print

// * Use a function to print the messages
fn print_message(gt_100: bool)
{
    // * Use a match expression to determine which message to print
    match gt_100 {
        true => println!("its big"),
        false => println!("its small"),
    }

}

fn main() {
    // * Use a boolean variable set to an expression
    //   that determines whether the value is >100 or <= 100
    let value = 100;
    let is_gt_100 = value > 100;
    print_message(is_gt_100);
}