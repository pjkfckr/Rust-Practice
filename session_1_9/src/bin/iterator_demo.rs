
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // let mut plus_one = vec![];
    //
    // for num in numbers {
    //     plus_one.push(num + 1);
    // }

    // collect() 를 호출하면 값들이 벡터로 변환됩니다.
    // 반복자는 그 자체로 아무것도 하지 않으니 collect()가 필수입니다.
    // collect()를 호출하거나 for 문을 수행해야 어떤 동작을 할 수 있습니다
    // 한마디로 계획 같은 거라고 보면 됩니다.
    // .iter()는 numbers의 원소를 훑을 계획이고, 그때마다 어떤 작업을 하겠다고 프로그램에 언질을 주는 겁니다.
    // 원소에 12을 더하는 .map()을 수행하겠다고 말이죠
    // 그리고 작업을 마치면 collect() 까지 수행합니다.
    // collect()는 원소를 새 벡터에 담습니다.
    // 타입 주석에 명시했듯이요
    let plus_one: Vec<_> = numbers
        .iter()
        .map(|num| num + 1)
        .filter(|num| num < 1)
        .collect();

    let new_numbers: Vec<_> = numbers
        .iter()
        .filter(|num| num <= &3)
        .collect();

    let numbers = vec![];
    let find_me: Option<&i32> = numbers
        .iter()
        .find(|num| num == &3);

    let count = numbers
        .iter()
        .count();

    let last: Option<&i32> = numbers
        .iter()
        .last();

    let numbers = vec![1, 2, 3, 4, 5];
    let min: Option<&i32> = numbers
        .iter()
        .min();

    let take: Vec<_> = numbers
        .iter()
        .take(3)
        .collect();
}