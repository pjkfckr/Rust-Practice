
struct Favorite {
    color: &'static str,
    number: i32,
    state: &'static str,
    food: &'static str,
    hobby: &'static str,
    place: &'static str
}

fn main() {
    let coord = (2, 3);
    println!("{:?}, {:?}", coord.0, coord.1);

    let (x, y) = (2, 3);
    println!("{:?}, {:?}", x, y);

    let (name, age) = ("Emma", 20);

    // Bad Example
    // index 0..5
    let favorites = ("red", 14, "TX", "pizza", "TV SHOW", "home");
    let state = favorites.2; // tx = index of 2
    let place = favorites.5; // home = index of 5

    // Good Example
    let favorite = Favorite {
        color: "red",
        number: 14,
        food: "pizza",
        state: "TX",
        hobby: "TV SHOW",
        place: "home"
    };
    let state = favorite.state;
    let place = favorite.place;

}