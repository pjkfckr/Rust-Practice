

// Topic: Organizing similar data using structs

// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink  flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounce
// * Use a match expression to print the drink flavor

// * Use an enum to create different flavors of drinks
enum Flavor {
    Sparkling,
    Sweet,
    Fruity,
}

// * Use a struct to store drink  flavor and fluid ounce information
struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}

// * Use a function to print out the drink flavor and ounce
fn print_drink(drink: Drink) {
    // * Use a match expression to print the drink flavor
    match drink.flavor {
        Flavor::Sparkling => println!("flavor: sparkling"),
        Flavor::Sweet => println!("flavor: sweet"),
        Flavor::Fruity => println!("flavor: fruity")
    }
    println!("oz: {:?}", drink.fluid_oz);

}


fn main() {
    // Sweet Drink
    let sweet = Drink {
        flavor: Flavor::Sweet,
        fluid_oz: 6.0
    };
    // Sparking Drink
    let sparking = Drink {
        flavor: Flavor::Sparkling,
        fluid_oz: 3.4
    };
    // Fruity Drink
    let fruity = Drink {
        flavor: Flavor::Fruity,
        fluid_oz: 7.3
    };

    print_drink(sweet);
    print_drink(sparking);
    print_drink(fruity);
}