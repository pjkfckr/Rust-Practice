
// Expressions
// - Rust는 식에 기반한 언어로, 곧 대부분의 것들이 평가되어 값을 반환한다는 의미입니다
// - 이러한 식 값들은 하나의 점으로 병함되므로 중첩 로직에 식을 사용할 수 있습니다

enum Menu {
    Burger,
    Fries,
    Drink
}

fn main() {
    let my_num = 3;
    let is_lt_5 = if my_num < 5 {
        true
    } else {
        false
    };

    let is_lt_5 = my_num < 5;

    let message = match my_num {
        1 => "hello",
        _ => "goodbye"
    };

    let paid = true;
    let item = Menu::Drink;
    let drink_type = "water";
    let order_placed = match item {
        Menu::Drink => {
            drink_type == "water"
        },
        _ => true
    };
}

// Recap
// - 식을 쓰면 if와 매칭식을 중첩해 로직을 사용할 수 있게 되지만
//   두세단계 이상 중첩하게 되면 코드가 굉장히 난잡해집니다