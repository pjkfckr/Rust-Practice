
// Type Annotations
// - 타입 주석은 함수 시그니처에 사용됩니다.
//   그 밖의 타입은 대부분 컴파일러가 추론합니다
// - 하지만 필요하다면 특정 위치에 명시적으로 타입을 지정할 수도 있습니다
//   이것을 "명시적 타입 주석(지정)" 이라고 합니다 (Explicit type annotations)


// Example - Basic
// 아래와 같이 함수가 있고 타입이 표시되어있는 전체를 "함수 시그니쳐" 라고 부릅니다.
// 시그니쳐 안에는 2가지의 타입이 있습니다.
// 하나는 문자열 슬라이스고, 하나는 32비트 정수형 입니다. 함수에서는 항상 이렇게 타입 주석으로
// 자료형을 표시해야합니다.
fn print_many(msg: &str, count: i32) {

}

enum Mouse {
    LeftClick,
    RightClick,
    MiddleClick
}

fn main() {
    // 아래의 변수 타입 주석을 작성안하여 사용해도 컴파일에서 자동으로 타입을 추론합니다.
    // 하지만 미리 명시해놓고 다른걸 작성 못하게 할수도있죠.
    // 반드시 명시해야 하는 경우도 있습니다.
    // 러스트 컴파일러가 변수의 타입을 추론할 수 없는 경우에는 명시해야 합니다.
    // 그런 경우에는, 타입을 알아낼 수 없기 때문에 타입 지정이 필요하다는 에러 메세지가 나올것입니다.
    let num: i32 = 15;
    let a: char = 'a';
    let left_click: Mouse = Mouse::LeftClick;

    // Example - Generics
    // 구조체에서 벡터를 사용하려면, 시그니처 지정 방법을 알아야합니다.
    // 벡터를 만들 때는, 벡터에 저장할 데이터의 자료형을 <>안에 지정할 수 있습니다.
    let numbers: Vec<i32> = vec![1, 2, 3];
    let letters: Vec<char> = vec!['a', 'b'];
    let clicks: Vec<Mouse> = vec![
        Mouse::LeftClick,
        Mouse::RightClick,
        Mouse::MiddleClick
    ];
}

// Recap

// 함수 본문에는 타입 주석이 필요하지 않은 경우가 대부분입니다.
// 하지만 컴파일러가 타입을 추론할 수 없는 경우에는 반드시 추가해야 해요.
// 이련경우에는 컴파일러가 타입을 추론할 수 없기 때문에,
// 타입 주석으로 자료형을 직접 지정해야 한다는 메세지가
// let 바인딩으로 변수를 선언할 때 타입 주석을 붙이는 것 역시 선택사항 입니다.
// 물론, 컴파일러가 알아서 처리하게 두는 편이 더 간단하죠