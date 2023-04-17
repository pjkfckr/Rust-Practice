

// Derive(파생) 매크로
// "derive"는 열거형 및 구조체 자료에 쓸 수 있는 특수 매크로입니다
// - Debug 는 구조체 또는 열거형에서 "{:?}" 형식의 디버그 토큰을 사용할 수 있도록 지원한다.
//   구조체에서는 모든 필드에 추가하지 않으면 컴파일 에러가 발생한다
// - Copy 또는 Clone 매크로를 추가하면, 컴파일러에게 이 열거형은 구조체에 저장하거나 함수에 넘길 때,
//   사본을 만들어서 넘기도록 지시합니다. 즉, Position 값을 함수나 구조체에 넘길 때, 소유권은 이동하지 않고
//   복사된 값이 넘어가는 겁니다.
// - Copy 와 Clone은 규모가 작은 구조체에서만 사용합니다. 구조체에 엔트리가 네다섯 개 정도 있을 때만 사용하죠
//   구조체에 데이터가 많을 때는 자동 복사 기능을 사용하게되면, 복사본이 많아진다면 모르는 사이 메모리를 많이 사용하게 된다.
//

#[derive(Debug, Copy, Clone)]
enum Position {
    Manager,
    Supervisor,
    Worker,
}

#[derive(Debug, Copy, Clone)]
struct Employee {
    position: Position,
    work_hours: i64
}

fn print_employee(emp: Employee) {
    println!("{:?}", emp);
}

fn main() {
    let me = Employee {
        position: Position::Worker,
        work_hours: 40
    };
    // 위와 같이 Clone, Copy 매크로를 추가하고 난 후에는
    // &로 참조하지않아도 소유권이 바뀌지않게 됩니다.
    // 밑의 me는 복사본 입니다.
    print_employee(me);
    // 또 한번 호출하면 복사본이 하나 더 생깁니다.
    print_employee(me);
}