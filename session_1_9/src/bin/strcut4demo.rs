

// Structure
//
//
// - struct 는 Structure 의 줄입말로 여러 데이터 조각을 포함하는 데이터 타입입니다.
//   * 구조체는 있거나 아예 없거나 둘중 하나입니다.
//   * 구조체에 포함된 각 데이터 조각은 반드시 채워져 있어야 하며
//   * 구조체의 일부만 갖거나 일부만 갖지 않을 수는 없습니다.
// - 구조체에 포함된 각각의 데이터 조각을 필드라고 부릅니다.
// - 구조체는 데이터로 작헙하는 것을 수월하게 만들어 줍니다. 프로그램에서 유사한 데이터끼리 묶을 수 있고
//   코드의 다른 부분으로 함께 이동시킬 수 있기 때문입니다.


// Example
struct ShippingBox {
    depth: i32,
    width: i32,
    height: i32,
}

struct GroceryItem {
    stock: i32,
    price: f64,
}

fn main() {
    let my_box = ShippingBox {
        depth: 3,
        width: 2,
        height: 5
    };

    let cereal = GroceryItem {
        stock: 10,
        price: 2.99,
    };

    let tall = my_box.height;
    println!("the box is {:?} units tall", tall);
    println!("stock: {:?}", cereal.price);
}


// Recap
//
// - 구조체는 여러 데이터 조각을 처리합니다.
// - 구조체 안에 있는 모든 필드는 구조체를 생성하기 위해 반드시 있어야 하며,
// - dot(.) 을 이용해 구조체의 모든 필드에 접근할 수 있습니다.
