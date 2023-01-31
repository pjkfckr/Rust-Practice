use crate::greet::hello;

mod greet {
    pub(crate) fn hello() {
        println!("hello");
    }

    pub(crate) fn goodbye() {
        println!("goodbye");
    }
}

mod math {
    pub(crate) fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub(crate) fn sub(a: i32, b: i32) -> i32 {
        a - b
    }
}


fn main() {
    use greet::hello;
    hello();
    greet::goodbye();
    math::add(1, 1);
}
