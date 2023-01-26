

// Topic: Browsing standard library documentation

// Requirements:
// * Print a string in lowercase and uppercase

// Notes:
// * Utilize standard library functionality to
//   transform the string to lowercase and uppercase
// * Use "rustup doc" in a terminal to open the standard library
// * Navigate to the API documentation section
// * Search for functionality to transform a string (or str)
//   to uppercase and lowercase
// * Try searching for: to_uppercase, to_lowercase

fn main(){
    let lowercase_string = "lowercase string";
    let uppercase_string = "UPPERCASE STRING";
    println!("lower to upper: {:?}", lowercase_string.to_uppercase());
    println!("upper to lower: {:?}", uppercase_string.to_lowercase());
}