
// Hashmap
// * 해시맵은 찾고자 하는게 명확한 상태에서 정보를 저장하고 가져올 때 유용하게 사용됩니다.
// * 해시맵은 데이터를 키-값 쌍 형태로 저장하는 컬렉션입니다.
//   "key"는 데이터를 찾는 수단이고,
//   "value"는 데이터 그 자체를 의미합니다.
// * 사전의 정의와 비슷합니다.
//   우리는 사전을 사용할 때, 알고 있는 단어를 바탕으로 그 단어가 의미하는 바를 찾아봅니다.
//   해시맵도 같은 개념이에요
// 우리는 키를 알고 있지만, 키에 어떤 값이 있는지는 모릅니다.
// 키를 사용하면 해시맵에서 아주 빠르게 데이터를 검색할 수 있죠

use std::collections::HashMap;

fn main() {
    // Example: find data
    let mut people = HashMap::new();
    people.insert("Susan", 21);
    people.insert("Ed", 13);
    people.insert("Will", 14);
    people.insert("Cathy", 22);
    people.remove("Susan");

    match people.get("Ed") {
        Some(age) => println!("age = {:?}", age),
        None => println!("not found"),
    }
    
    // Example: iterate
    // iter() 메서드는 키와 값을 괄호로 묶어 튜플(tuple) 형태로 반환합니다.
    // 해시맵의 중요한 특징 중 하나는 데이터가 랜덤한 순서로 저장됩니다.
    for (person, age) in people.iter()  {
        println!("person = {:?}, age = {:?}", person, age);
    }

    for person in people.keys() {
        println!("person = {:?}", person);
    }

    for age in people.values() {
        println!("age = {:?}", age);
    }
}

// Recap
// * 해시맵은 키-값 쌍 형태로 데이터를 저장합니다. 키를 통해 값에 접근할 수 있죠\
// * 키를 사용해 빠르게 데이터를 추가하고 가져올 수 있습니다.
// * 위치를 기반으로 데이터를 가져오고 싶을 때 유용한 구조입니다.