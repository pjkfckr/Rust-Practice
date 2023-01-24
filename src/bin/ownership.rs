

// Managing memory

// - 모든 프로그램은 항상 메모리 사용을 추적해야 하며 이에 실패하면 메모리 누수가 발생합니다
// - 메모리 누수는 프로그램에서 사용되는 메모리 추적에 실패해서 새로운 메모리를 예약해야 하는 경우를 뜻합니다
// - 모든 프로그래밍 언어들은 메모리를 관리하는 각자의 방법이 있습니다.
// - Rust는 "ownership" 이라는 것을 이용하며
//   "ownership"이란 메모리의 소유자가 메모리를 정리하게 됨을 의미하며
//   Rust에서 소유자란 단순히 함수입니다.
//   메모리는 소유자로부터 이동하거나 대여할 수 있습니다.

// Example - Move
// 지금까지 우리의 프로그램에서 소유권을 이동해왔는데, 이 프로그램을 통해 살펴봅시다.
enum Light {
    Bright,
    Dull,
}

fn display_light(light: &Light) {
    match light {
        Light::Bright => println!("bright"),
        Light::Dull => println!("dull"),
    }
}

fn main() {
    let dull = Light::Dull;
    // 해당 코드는 에러가 발생하게 됩니다 ( value used here after move )
    // 이 에러는 "ownership" 떄문으로, Light을 만들어 변수 dull에 할당할 때
    // 소유자는 main함수가 됩니다
    // main 함수의 중괄호 안의 어떤 것이든 main함수가 소유하고 있는 것이죠
    // display_light(dull)을 호출하면 dull은 display_light으로 이동해서
    // fn display_light 함수에 위치하게 됩니다
    // display_light(dull);
    // 위와 같이 데이터가 display_light 함수로 이동했으므로 dull은 display_light의 소유입니다
    // 그리고 데이터를 소유한 모든 함수는 함수가 끝날 때 그 데이터를 삭제해야 합니다
    // 따라서 light은 main함수로부터 display_light 함수로 이동하며 display_light 함수가 끝날 때
    // 같이 삭제됩니다
    // 위의 함수에서 match expression 이 끝나 함수가 종료되면 light 가 삭제됩니다
    // light가 삭제된 뒤에는 같은 함수 호출에서 light을 더이상 사용할 수 없습니다
    // 따라서 첫번째 함수 호출에서는 dull이 함수로 이동하고 함수가 실행되면
    // dull은 새로운 소유자인 display_light에서 삭제되므로 더이상 사용할수 없게 됩니다
    // display_light(dull);
    // 그 후에 dull로 함수를 다시 호출하려 할 때 dull은 이전에 삭제됐으므로 존재하지 않습니다.
    // 컴파일러는 에러가 있음을 알리며 dull이 함수로 이동했고 이미 삭제돼서 같은 함수에서 더이상 사용할 수 없다고 알려줍니다
    // 여기서 중요하게 기억해야 할 점은 변수가 만들어진 곳이 최초의 소유자가 된다는 점입니다
    // 여기서 dull 변수의 소유자는 main함수 였다가 다른 함수를 그 데이터와 함께 호출하면
    // 소유자가 그 함수로 변경됩니다

    // display_light 함수를 두 번 사용할 수 없게되는 이 에어를 고치기 위해 대여라는 것을 해야합니다
    // 아리의 코드는 위의 코드와 거의 같은데 이동 대신 대여를 하도록 변경됐습니다
    display_light(&dull);
    display_light(&dull);
    // Rust에서 &는 데이터를 대여한다는 의미입니다.
    // 참조라고도 불려서 데이터를 참조한다거나 대여한다고 부를수 있습니다
}

// Recap

// - 메모리는 누수를 방지하기 위해 어떻게든 관리가 되어야 하는데
//   Rust는 "ownership"을 이용해 메모리를 관리합니다
// - 데이터의 소유자가 반드시 메모리를 정리해야 하며
//   이는 중괄호가 닫히는 스코프의 끝에서 자동적으로 일어납니다
// - 함수를 호출할 때의 기본 행동 양식은 메모리를 새로운 소유자로 옮기는 것입니다
//   데이터를 대여만 하고 싶다면 "&"를 써서 함수가 메모리를 대여하도록 합니다


