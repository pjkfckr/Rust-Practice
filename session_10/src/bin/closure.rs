


// Using dynamic keyword (dyn)
fn math(a: i32, b: i32, op: Box<dyn Fn(i32, i32) -> i32>) -> i32 {
    op(a, b)
}

fn main() {
    // 클로저는 환경을 애워싸서 전에 정의된 모든 것들은 클로저 안으로 들어가서
    // 접근 가능하게 될 것입니다.
    // move: Capture a closure's environment by value.
    // 이 클로저를 Box에 넣었기 때문에 키워드 move를 사용해야 합니다.
    // move 키워드를 사용하지 않으면 '`name` does not live long enough' 이라는 에러가 발생합니다
    // 이는 Box가 name에 대한 소유권이 없기때문에 발생하게 됩니다.
    let name = "Sherpa";
    // move 를 사용해 클로저 안으로 name을 이동시킵니다.
    // 만약 클로저가 박스에 있지 않다면, 이동할 필요가 없을 것이고
    // 아마 자동으로 이동할 수 있을겁니다.
    // 하지만 Box에 넣었기 때문에, 소유권을 이동해야 합니다.
    // 어떤 것을 Box에 넣을 때는 모든 것을 소유해야 하기 때문입니다.
    let add = Box::new(move |a,b| {
        println!("adding a number for {}!", name);
        a + b
    });
    let sub = Box::new(|a,b| a - b);
    let mul = Box::new(|a,b| a * b);
    println!("{}", math(2, 3, add));
    println!("{}", math(2, 3, sub));
    println!("{}", math(2, 3, mul));
}