

// Topic: Result & the question mark operator

// Requirements:
// * Determine if an employee can access building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position

// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

use crate::Status::Active;

// * Use an enum to represent all types of employees
enum Position {
    Maintenance,
    Marketing,
    Manager
}

enum Status {
    Active,
    Terminated
}

// * Use a struct to store the employee type and whether they are still employed
struct Employee {
    position: Position,
    status: Status
}

// * Use a function that returns a Result to determine if the employee
//   may enter the building
fn try_access(employee: &Employee) -> Result<(), String> {
    match employee.status {
        Status::Terminated => return Err("Terminated".to_owned()),
        _ => (),
    }

    match employee.position {
        Position::Maintenance => Ok(()),
        Position::Marketing => Ok(()),
        Position::Manager => Ok(()),
        _ => Err("Invalid position".to_owned()),
    }
}

// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this
fn print_access(employee: &Employee) -> Result<(), String> {
    let attempt_access = try_access(employee)?;
    println!("Allow access");
    Ok(())
}

fn main() {
    let manager = Employee { position: Position::Manager, status: Active };

    match print_access(&manager) {
        Err(error) => println!("{:?}", error),
        _ => (),

    }

}

