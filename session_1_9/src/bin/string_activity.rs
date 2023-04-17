

// Topic: Strings

// Requirements:
// * Print out the name and favorite colors of people aged 10 and under

// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function


// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
struct Person {
    age: i32,
    name: String,
    favorite_color: String
}

impl Person {
    fn new(age: i32, name: String, color: String) -> Self {
        Self {
            age,
            name,
            favorite_color: color
        }
    }

    fn print(&self) {
        println!("name: {:?}, favorite_color: {:?}", self.name, self.favorite_color)
    }

}

fn print_person(person: Person) {
    println!("age: {:?}, name: {:?}, color: {:?}", person.age, person.name, person.favorite_color);
}

fn print(data: &str) {
    println!("{:?}", data);
}

fn main() {
    // * Create and store at least 3 people in a vector
    // let people = vec![
    //     Person { age: 12, name: String::from("Jayson"), favorite_color: String::from("Red") },
    //     Person { age: 9, name: String::from("Park"), favorite_color: String::from("Brown") },
    //     Person { age: 7, name: String::from("Billy"), favorite_color: String::from("Blue") }
    // ];

    let people = vec![
        Person::new(12, String::from("Jayson"), String::from("Red")),
        Person::new(9, String::from("Park"), String::from("Brown")),
        Person::new(7, String::from("Billy"), String::from("Blue")),
    ];

    // * Iterate through the vector using a for..in loop
    for person in people {
        // * Use an if expression to determine which person's info should be printed
        // * The name and colors should be printed using a function
        if person.age <= 10 {
            // * The name and colors should be printed using a function
            print(&person.name);
            print(&person.favorite_color);
            person.print();
            print_person(person);
        }
    }
}





