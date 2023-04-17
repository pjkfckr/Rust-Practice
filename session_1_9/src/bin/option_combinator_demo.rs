

fn main() {
    let a: Option<i32> = None;
    dbg!(a);

    // 옵션 내부에 데이터가 있는지 확인
    let a_is_some = a.is_some();
    dbg!(a_is_some);
    // 옵션 내부에 데이터가 없는지 확인
    let a_is_none = a.is_none();
    dbg!(a_is_none);
    // 클로저를 받고, 첫 번째 인자는 옵션 내부의 값, 데이터가 존재할 때만 동작
    let a_mapped = a.map(|num| num + 1);
    dbg!(a_mapped);
    // 단일 함수 인자를 받는 클로저를 취하는데, 인자는 옵션의 내부 타입입니다
    // 반환값이 true 이면 옵션 데이터를 유지하고, 반환값이 false이면 데이터를 버립니다.
    let a_filtered = a.filter(|num| num == &1);
    dbg!(a_filtered);
    // 클로저를 받고, 이 클로저는 인자를 사용하지 않으니 || 를 적고, 데이터가 없다면 Some(data)를 반환
    // 데이터가 없을때 특정값을 줘야할때 사용
    let a_or_else = a.or_else(|| Some(5));
    dbg!(a_or_else);
    // ||를 인자로 받고, or_else 와 유사해보이지만, or_else는 옵션데이터를 반환하는데,
    // unwrap_or_else()를 호출하면 데이터를 꺼내 변수에 할당하기 때문에 a에 있던 옵션 데이터는사라집니다
    let unwrapped = a.unwrap_or_else(|| 0);
    dbg!(unwrapped);
}