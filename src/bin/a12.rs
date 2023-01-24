

// Topic: Implementing functionality with the impl keyword
// 주제는 impl 키워드를 이용한 기능 구현

// Requirements:
// * Print the characteristics of a shipping box
//   배송상자의 특성을 출력
// * Must include dimensions, weight, and color
//   이 특성에는 크기와 무게, 색상이 있다

// Notes:
// * Use a struct to encapsulate the box characteristics
//   구조체를 사용해서 특성들을 캡슐화하시오
// * Use an enum for the box color
//   색상에는 열거형을 이용하시오
// * Implement functionality on the box struct to create a new box
//   새로운 상자를 만들 수 있는 기능을 구현하시오
// * Implement functionality on the box struct to print the characteristics
//   상자 구조체에 상자의 특성을 출력하는 기능을 함수로 사용할 수 있도록 구현하시오


// * Use an enum for the box color
enum Color {
    Red,
    Brown
}

impl Color {
    fn print(&self) {
        match self {
            Color::Red => println!("red"),
            Color::Brown => println!("brown"),
        }
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64
}

impl Dimensions {
    fn print(&self) {
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);
    }
}

// * Use a struct to encapsulate the box characteristics
struct ShippingBox {
    color: Color,
    weight: f64,
    dimensions: Dimensions
}

// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
impl ShippingBox {
    // Rust에서 함수 이름을 짓는 규칙에 따르면 새로운 구조체나 열거형을 만드는 기능을
    // 구현할 때 new라는 함수 이름을 사용합니다
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions
        }
    }
    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("weight: {:?}", self.weight);
    }
}

fn main() {
    let small_dimensions = Dimensions {
        width: 1.0,
        height: 2.0,
        depth: 3.0,
    };
    let small_box = ShippingBox::new(5.0, Color::Brown, small_dimensions);
    small_box.print();
}