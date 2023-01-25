


// Enums
// - 열거형은 한 항목이 한 가지 값을 나타내는 자료형입니다.
//   각 항목을 variant*열것값) 이라고 부릅니다.
// - 열거형에는 1차원적인 값만 저장할 수 있는 게 아닙니다
//   열거형의 각 variant 에는 부가 데이터를 함께 저장할 수 있습니다

// Example
#[derive(Debug)]
enum Mouse {
    LeftClick,
    RightClick,
    MiddleClick,
    Scroll(i32),
    Move(i32, i32)
}

#[derive(Debug)]
enum PromoDiscount {
    NewUser,
    Holiday(String),
}

#[derive(Debug)]
enum Discount {
    Percent(f64),
    Flat(i32),
    Promo(PromoDiscount),
    Custom(String)
}

fn main() {
    let moved = Mouse::Move(-1, -1);
    let promotion = Discount::Promo(
        PromoDiscount::Holiday(
            String::from("friday")
        )
    );
}

// Recap
// - 열것값에는 연관 값을 추가할 수 있습니다.
//   다른 열거형을 값으로 가질 수도 있죠
// - 동일한 열거형에 일반 식별자와 연관 값을 갖는 열것값을 모두 추가할 수도 있습니다.
// - 하나의 열것값이 두 개 이상의 연관 값을 가질 수도 있습니다.

