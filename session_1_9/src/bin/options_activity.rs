

// Topic: Option

// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students

// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>
// * Lockers use numbers and are optional for students
struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let jayson = Student {
        name: "Jayson".to_owned(),
        locker: Some(15),
    };
    println!("student name: {:?}", jayson.name);
    // * Print out the details of a student's locker assignment
    match jayson.locker {
        Some(num) => println!("locker number: {:?}", num),
        None => println!("locker not provided")
    }
}