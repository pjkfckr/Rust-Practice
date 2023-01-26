

// Documentation

// --open: 문서화가 끝난 후 문서화된 파일을 자동으로 열어줍니다.
// command: $ cargo doc --open
/// A favorite color.
enum Color {
    Red,
    Blue,
}

/// A piece of mail.
struct Mail {
    /// The destination address.
    address: String,
}

/// Adds two numbers together.
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {

}