
//  Topic: New type pattern

//  Requirements:
//  * Display the selected color of shoes, a shirt, and pants
//  * Create and display at least one of each type of clothes and color

//  Notes:
//    옷의 색상에 대해 new 타입을 생성하고 각각의 new 타입은
//    new() 함수를 구현해야한다
//  * Create a new type for the colors of the clothes
//    * Each new type should implement a `new` function
//  * Create a function for each type of clothes that accepts a
//    new type color specific to that type of clothing

#[derive(Debug)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Read,
    White,
    Yellow
}

#[derive(Debug)]
struct ShirtColor(Color);

impl ShirtColor {
    fn new(color: Color) -> Result<Self, String> {
        match color {
            Color::Purple => Err("purple not allowed".to_owned()),
            other => Ok(Self(other))
        }
    }
}

#[derive(Debug)]
struct ShoesColor(Color);

impl ShoesColor {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

#[derive(Debug)]
struct PantsColor(Color);

impl PantsColor {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

fn print_shirt_color(color: Result<ShirtColor, String>) {
    println!("shirt color = {:?}", color);
}

fn print_shoes_color(color: ShoesColor) {
    println!("shoes color = {:?}", color);
}

fn print_pants_color(color: PantsColor) {
    println!("pants color = {:?}", color);
}


fn main() {
    let shirt_color = ShirtColor::new(Color::Black);
    let shoes_color = ShoesColor::new(Color::Custom(String::from("Winter")));
    let pants_color = PantsColor::new(Color::Purple);

    print_shirt_color(shirt_color);
    print_shoes_color(shoes_color);
    print_pants_color(pants_color);
}