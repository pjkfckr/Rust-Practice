
fn maybe_num() -> Option<i32> {
    None
}

fn maybe_word() -> Option<String> {
    None
}

fn main() {
    // let plus_one = match maybe_num() {
    //     Some(num) => Some(num + 1),
    //     None => None
    // };
    // // map() 함수는 작업할 함수를 받습니다.
    // // 클로저는 수직 막대 또는 파이프를 이용해서 만들고, 그 파이프들 사이에 인자를 넣어줍니다.
    // // map()이 좋은 점은 실제로 거기에 값이 있을 때만 적용됩니다.
    // // 그래서 map()을 사용하고 Option 이 None 이면 코드가 실행되지 않습니다.
    // let plus_one = maybe_num().map(|num| num + 1);

    let word_length: Option<usize> = maybe_word()
        .map(|word| word.len())
        .map(|len| len * 2);
}