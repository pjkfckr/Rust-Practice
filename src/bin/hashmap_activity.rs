use std::collections::HashMap;

// Topic: HashMap

// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock

// Notes:
// * Use a HashMap for the furniture store stock


fn main() {
    // * Use a HashMap for the furniture store stock
    let mut stock = HashMap::new();
    stock.insert("Chair", 5);
    stock.insert("Bed", 3);
    stock.insert("Table", 2);
    stock.insert("Couches", 0);
    let mut total_stock = 0;

    for num in stock.values() {
        if num == &0 {
            println!("out of stock");
        } else {
            println!("number = {:?}", num);
        }
    }

    for (item, quantity) in stock.iter() {
        total_stock = total_stock + quantity;
        let stock_count = if quantity == &0 {
            "out of stock".to_owned()
        } else {
            format!("{:?}", quantity)
        };
        println!("item = {:?}, quantity = {:?}", item, stock_count);
    }

    println!("Total Quantity = {:?}", total_stock);
}