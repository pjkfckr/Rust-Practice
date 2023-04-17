use std::collections::HashMap;

#[derive(Debug)]
struct Contents {
    content: String
}

fn main() {
    // mutable로 선언하지 않으면 변수에 아무것도 추가할 수 없다
    let mut lockers = HashMap::new();
    lockers.insert(1, Contents { content: "stuff".to_owned() });
    lockers.insert(2, Contents { content: "shirt".to_owned() });
    lockers.insert(3, Contents { content: "gym shorts".to_owned() });

    for (locker_number, content) in lockers.iter() {
        println!("number: {:?}, content: {:?}", locker_number, content);
    }
}