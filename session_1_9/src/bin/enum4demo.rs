

// Enum(Enumeration)
// 열거형은 여러 가능한 값 중 하나의 데이터 조각입니다
// 각각의 가능한 값을 열거값이라고 부릅니다

// 열거형은 컴파일러에 프로그램에 관한 정보를 제공하므로 프로그램을 더 안정적으로 만들 수 있습니다

// Example
enum Direction {
    Up,
    Down,
    Left,
    Right
}
fn which_way(go: Direction) -> &'static str {
    match go {
        Direction::Up => "up",
        Direction::Down => "down",
        Direction::Left => "left",
        Direction::Right => "right",
    }
}

// Recap
//
// 열거형은 한 번에 하나의 열것값만 될 수 있습니다.
// 그리고 키워드와 함께 사용하면 프로그램을 더 안정적으로 만들어줍니다.
// 또한, 프로그램의 코드를 좀더 읽기 쉽게 도와줍니다.

fn main() {
    which_way(Direction::Up);
}