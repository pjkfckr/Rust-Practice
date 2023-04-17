
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

//      * Off
//      * Sleep
//      * Reboot
//      * Shutdown
//      * Hibernate
enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

//  * Use a match expression to convert the user input into the power state enum
impl PowerState {
    fn new (state: &str) -> Option<PowerState> {
        let state: String = state.trim().to_lowercase();
        // String -> &str
        match state.as_str() {
            "off" => Some(PowerState::Off),
            "sleep" => Some(PowerState::Sleep),
            "reboot" => Some(PowerState::Reboot),
            "shutdown" => Some(PowerState::Shutdown),
            "hibernate" => Some(PowerState::Hibernate),
            _ => None
        }
    }
}

//  * Use a function with a match expression to print out the power messages
//      * The function should accept the enum as an input
fn print_power_action(state: PowerState) {
    // use라는 키워드는 표준 라이브러리의 모듈을 사용하고,
    // 코드의 다른 부분에 있는 다른 아이템들을 사용하기 위해서도 사용할 수 있습니다.
    // use PowerState라고 하면 enum PowerSate를 사용하게 됩니다
    // ::는 항상 경로 횡단을 의미합니다.
    // PowerState 에서 각각의 열것값으로 가게 됩니다.
    use PowerState::*;
    match state {
        Off => println!("turning off"),
        Sleep => println!("sleeping"),
        Reboot => println!("rebooting"),
        Shutdown => println!("shutting down"),
        Hibernate => println!("hibernating")
    }
}

fn main() {
    let mut buffer = String::new();
    let user_input_status = io::stdin().read_line(&mut buffer);
    if user_input_status.is_ok() {
        match PowerState::new(&buffer) {
            Some(state) => print_power_action(state),
            None => println!("invalid power state")
        }
    } else {
        println!("error reading input");
    }


}