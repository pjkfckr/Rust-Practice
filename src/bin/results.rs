

// Result
// * Result 타입은 두 종류의 값을 가질 수 있습니다.
//   성공한 경우에 갖는 값이 있고,
//   실패한 경우에 갖는 값이 있습니다.

// * 이 타입은 실패할 가능성이 있는 동작을 수행해야 할 때 사용합니댜.
//   예를 들면, "파일 복사" "웹사이트 연결" 같은 경우입니다.

// Definition
// enum Result<T, E> {
//     Ok(T),
//     Err(E)
// }
// Option 과 마찬가지로 열거형이며 두 개의 열것값을 갖고 있어요


// Example
struct SoundData {
    sound: String
}

impl SoundData {
    fn new(sound: String) -> Self {
        Self {
            sound
        }
    }
}

fn get_sound(name: &str) -> Result<SoundData, String> {
    if name == "alert" {
        Ok(SoundData::new(String::from(name)))
    } else {
        Err("unable to find sound data".to_owned())
    }
}

fn main() {
    let sound = get_sound("alert");
    match sound {
        Ok(_) => println!("sound data located"),
        Err(e) => println!("error: {:?}", e),

    }
}

// Recap
// * Result 타입은 성공 또는 실패를 나타냅니다.
//   Ok 열거값은 동작이 성공적으로 실행됐음을 의미하고,
//   Err 열거값은 동작이 실패했음을 의미합니다.
// * 이 타입은 실패할 가능성이 있는 동작을 수행하는 함수에 사용합니다.
// * Result 를 사용하려면 성공했을 때 반환할 타입과 실패했을 때 반환할 타입을 지정해줘야한다.
//   Result<T, E>