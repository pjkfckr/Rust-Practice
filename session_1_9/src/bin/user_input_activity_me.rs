
// Topic: User Input

// Requirements:
//  * Verify user input against pre-defined keywords
//  * The keywords represent possible power options for a computer:
//      * Off
//      * Sleep
//      * Reboot
//      * Shutdown
//      * Hibernate
//  * If the user enters one of the keywords,
//    a message should be printed to the console indicating which action will be taken
//    * Example: If the user types in "shutdown" a message should display such
//      as "shutting down"
//  * If the keyword entered does not exist, an appropriate error message
//    should be displayed

//  Notes:
//  * Use an enum to store the possible power states
//  * Use a function with a match expression to print out the power messages
//      * The function should accept the enum as an input
//  * Use a match expression to convert the user input into the power state enum
//  * The program should be case-insensitive (the user should be able to type
//    Reboot, reboot, REBOOT, etc.)

use std::io;

//  * Use an enum to store the possible power states
#[derive(Debug)]
enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate
}

//  * Use a function with a match expression to print out the power messages
fn print_state(state: PowerState) {
    match state {
        PowerState::Off => println!("off"),
        PowerState::Sleep => println!("sleep"),
        PowerState::Reboot => println!("reboot"),
        PowerState::Shutdown => println!("shutting down"),
        PowerState::Hibernate => println!("hibernate"),
    }
}
//  * Use a match expression to convert the user input into the power state enum
fn word_to_state(word: String) {
    //  * The program should be case-insensitive (the user should be able to type
    //    Reboot, reboot, REBOOT, etc.)
    let lower_word = word.to_lowercase();
    match lower_word.as_str() {
        "off" => print_state(PowerState::Off),
        "sleep" => print_state(PowerState::Sleep),
        "reboot" => print_state(PowerState::Reboot),
        "shutdown" => print_state(PowerState::Shutdown),
        "hibernate" => print_state(PowerState::Hibernate),
        _ => println!("not allowed state"),
    }
}

fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}

fn main() {
    let mut times_input = 0;
    while times_input < 1 {
        match get_input() {
            Ok(words) => {
                word_to_state(words);
                times_input += 1;
            }
            Err(error) => println!("error: {:?}", error),
        }
    }
}