use demo::print_from_lib;
use demo::helpers;
use demo::group;

fn main() {
    print_from_lib();
    group::g1::g1_hello();
    helpers::print_again();
}