

// Topic: Looping using the while statement
//
//
// Notes:
// * Use a mutable integer variable
// * Use a while statement
// * print the variable within the while loop
// * Do not use break to exit the loop

fn main() {
    let mut counter = 5;

    while counter >= 1 {
        println!("{:?}", counter);
        counter = counter - 1;
    }
    println!("done!");
}