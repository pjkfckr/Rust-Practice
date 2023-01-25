
// Option
// * Option 타입이라는건 두 가지 중 하나입니다.
//   자료형을 가진 데이터(값)가 있거나, 값이 없는거죠
// * 반드시 필요한 값이 아닌 경우와 값이 없을 수도 있는 경우에 사용합니다.
// * 사용 예시
//   - "값을 찾지 못했을 때"
//   - "목록에 추가된 항목 없을 때"
//   - "값이 채워지지 않은 폼 필드"


// Definition
// Option 타입은 러스트 프로그래밍 라이브러리에 포함돼 있습니다.
// 두 개의 열것값을 가진 열거형으로
// 첫 번째 옵션은 "Some"으로, 데이터가 있음을 나타냅니다.
// 두 번째 옵션은 "None"으로, 데이터가 없음을 나타냅니다.
// enum Option<T> {
//     Some(T),
//     None
// }


// Example
struct Customer {
    age: Option<i32>,
    email: String
}


fn main() {
    let mark = Customer {
        age: Some(22),
        email: "mark@example.com".to_owned(),
    };
    let becky = Customer {
        age: None,
        email: "becky@example.com".to_owned()
    };

    match becky.age {
        Some(age) => println!("customer is {:?} years old", age),
        None => println!("customer age not provided")
    }
}