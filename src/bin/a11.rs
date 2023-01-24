
// Topic: Ownership

struct GroceryItem {
    quantity: i32,
    id: i32
}

fn display_quantity(item: &GroceryItem) {
    println!("quantity: {:?}", item.quantity);
}

fn display_id(item: &GroceryItem) {
    println!("id: {:?}", item.id)
}

fn main() {
    let my_item = GroceryItem {
        quantity: 3,
        id: 99,
    };

    display_quantity(&my_item);
    display_id(&my_item);
}