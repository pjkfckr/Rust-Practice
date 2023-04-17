


// Topic: Adanved Match

// Requirements:
// * Print out a list tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price

// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

// * Use an enum for the tickets with data associated with each variant
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
enum Ticket {
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String),
}

fn main() {
    // * Create one of each ticket and place into a vector
    let tickets: Vec<Ticket> = vec![
        Ticket::Standard(1.99),
        Ticket::Backstage(3.14, String::from("Jayson")),
        Ticket::Vip(100.0, String::from("Jackson")),
    ];
    // * Use a match expression while iterating the vector to print the ticket info
    for ticket in tickets {
        match ticket {
            Ticket::Backstage(price, holder) => {
                println!("Holder: {:?}, price: {:?}", holder, price)
            },
            Ticket::Vip(price, holder) => {
                println!("Holder: {:?}, price: {:?}", holder, price)
            },
            Ticket::Standard(price) => println!("Price: {:?}", price)
        }
    }
}