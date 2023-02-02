

fn main() {
    // ..=는 마지막값을 포함
    // ..는 마지막값을 포함하지 않음
    let range = 1..=3;
    let range = 1..4;

    for num in 1..4 {
        println!("{:?}", num);
    }

    for ch in 'a'..='f' {
        println!("{:?}", ch);
    }
}