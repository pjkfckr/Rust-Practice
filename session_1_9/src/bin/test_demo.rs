
fn all_caps(word: &str) -> String {
    word.to_uppercase()
}

fn main() {

}

#[cfg(test)]
mod test {
    // Rust 에서는 코드 모음을 크레이트(crate) 라고 부릅니다.
    use crate::all_caps;

    #[test]
    fn check_all_caps() {
        let result = all_caps("hello");
        let expected = String::from("HELLO");
        assert_eq!(result, expected, "string should be all uppercase");
    }
}