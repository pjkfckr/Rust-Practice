
//  Topic: Option combinators

//  Requirements:
//  * Use combinators as described in the functions:
//    part_1, part_2, and part_3

//  Notes:
//  * Run `cargo test --bin option_combinator_activity` to check your program.

#[derive(Debug, Eq, PartialEq)]
enum Access {
    Admin,
    User,
    Guest
}

fn maybe_access(name: &str) -> Option<Access> {
    match name {
        "admin" => Some(Access::Admin),
        "gary" => Some(Access::User),
        _ => None
    }
}

fn root() -> Option<Access> {
    Some(Access::Admin)
}

fn part_1() -> bool {
    // We are checking whether or not this particular user
    // has an access level. The "admin" user does have
    // an access level.
    // Note: Use is_some or is_none.
    maybe_access("admin").is_some()
}

fn part_2() -> Option<Access> {
    // "Root" is equivalent to Access:Admin, but it is
    // not listed in the maybe_access function.
    // Note: Use or_else and root()
    maybe_access("root").or_else(|| root())
}

fn part_3() -> Access {
    // "Alice" is not a listed user, so she will be a guest.
    // Note: Use unwrap_or_else.
    maybe_access("Alice").unwrap_or_else(|| Access::Guest)
}

fn main() {

}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn check_part_1() {
        let result = part_1();
        let expected = true;
        assert_eq!(result, expected, "should be true");
    }

    #[test]
    fn check_part_2() {
        let result = part_2();
        let expected = Some(Access::Admin);
        assert_eq!(result, expected, "should be Some(Access::Admin)");
    }

    #[test]
    fn check_part_3() {
        let result = part_3();
        let expected = Access::Guest;
        assert_eq!(result, expected, "should be Access::Guest")
    }
}
