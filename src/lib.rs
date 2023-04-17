pub mod helpers;
pub mod group;

pub fn print_from_lib() {
    use helpers::{print_from_helper, print_again};
    println!("hello from lib");
    print_from_helper();
    print_again();
    group::g1::g1_hello();
}

