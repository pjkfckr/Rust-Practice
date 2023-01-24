

// Vector
// 벡터는 여러 데이터 조각을 저장할 수 있는 데이터 구조입니다
// 데이터는 동일한 자료형이여야 하며, 숫자형 문자형, 열거형, 또는 프로그램에서 만든
// 구조를 저장할 수 있습니다
// 벡터는 정보 목록에 사용됩니다. 예를 들어 식료품 목록이 있는 경우 모든 식료품 목록을 벡터에 저장할 수 있습니다
// 벡터를 사용하면 데이터를 추가하고 제거할 수 있습니다.
// 또한 데이터 작업을 위해 벡터의 항목을 쉽게 탐색할 수도 있습니다.

// Example
fn main() {
    let my_numbers = vec![1, 2, 3];
    let mut my_numbers = Vec::new();
    my_numbers.push(1);
    my_numbers.push(2);
    my_numbers.push(3);
    my_numbers.pop();
    my_numbers.len();

    let two = my_numbers[1];

    for num in my_numbers {
        println!("{:?}", num);
    }
}

// Recap
// - 벡터는 여러개의 유사한 데이터를 포함할 수 있습니다
// - 벡터에는 데이터를 쉽게 추가하거나 제거할 수 있습니다
// - 매크로를 사용하면 코드에서 벡터를 쉽게 생성할 수 있습니다
// - for in을 사용해 벡터의 항목을 반복할 수 있습니다