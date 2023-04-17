

// Advanced Match

enum Discount {
    Percent(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: f32,
}

fn main() {
    let n = 3100000;
    match n {
        3 => println!("three"),
        other => println!("number: {:?}", other),
    }

    let flat = Discount::Flat(30);
    match flat {
        Discount::Flat(2) => println!("flat 2"),
        Discount::Flat(amount) => println!("flat discount of {:?}", amount),
        _ => (),
    }

    let concert = Ticket {
        event: String::from("concert"),
        price: 30.0,
    };

    match concert {
        Ticket { price: 30.0, event } => println!("event @ 50 = {:?}", event),
        Ticket { price, .. } => println!("price = {:?}", price),
    }
}
