

//  Traits
//  * traits 란 한마디로 특정 기능의 존재를 명시한 것으로,
//    여러 타입에 걸쳐 기능을 표준화할 때 씁니다.
//  * 표준화를 거치면 함수를 여러 타입에 적용할 수 있습니다.
//    덕분에 코드 중복을 제거할 수 있습니다
//  * 원래 여러 타입으로 작업하려면 함수 역시 여러 개를 작성해야하는데
//    타입들의 기능이 비슷할 경ㄹ우 trait 하나로 대신할 수 있습니다.
//    trait 에 동작할 함수 하나만 작성하면, 타입들이 해당 trait 를 구현하는 한
//    그 함수 하나만 호출하면 됩니다.

// Example

trait Noise {
    fn make_noise(&self);
}

struct Person;

impl Noise for Person {
    fn make_noise(&self) {
        println!("hello");
    }
}
struct Dog;
impl Noise for Dog {
    fn make_noise(&self) {
        println!("woof");
    }
}

fn hello(noisy: impl Noise) {
    noisy.make_noise();
}

fn main() {
    hello(Person {});
    hello(Dog {});
}

//  Recap

//  * trait 란 여러 타입이 가진 비슷한 기능을 정의한 것으로, trait 의 함수는
//    일반 함수처럼 인자를 받고 값을 반환할 수 있습니다
//  * trait 함수는 대부분 &self 를 인자로 취합니다
//  * trait 를 거쳐 데이터를 넘길 때는 impl Trait 로 만들면 됩니다.